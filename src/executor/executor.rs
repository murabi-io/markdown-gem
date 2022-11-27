use std::{
    process::{ExitStatus, Stdio},
    thread,
};

use anyhow::{anyhow, Result};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, BufReader},
    process::{Child, Command},
    sync::oneshot,
    task::JoinHandle,
};

use crate::executor::command_output::{CommandExecInfo, CommandOutputLine, CommandStream};
use crate::executor::execution_plan::{ExecutionItem, ExecutionPlan};
use crate::executor::job::Job;
use crate::executor::job_location::JobLocation;
use crate::*;

/// an executor calling a command in a separate
/// thread when asked to and sending the lines of output in a channel,
/// and finishing by None.
/// Channel sizes are designed to avoid useless computations.
pub struct Executor {
    pub line_receiver: crossbeam::channel::Receiver<CommandExecInfo>,
    stop_sender: oneshot::Sender<()>, // signal for stopping the thread
    thread: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Task {
    pub backtrace: bool,
}

type LineSender = crossbeam::channel::Sender<CommandExecInfo>;

impl Executor {
    /// launch the commands, send the lines of its stderr/out on the
    /// line channel.
    pub fn new(
        location: JobLocation,
        mut execution_plan: ExecutionPlan,
        keep_files: bool,
    ) -> Result<Self> {
        let (stop_sender, mut stop_receiver) = oneshot::channel();
        let (line_sender, line_receiver) = crossbeam::channel::unbounded();

        let thread = thread::spawn(move || {
            // start a runtime to manage the executor
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .build()
                .unwrap();

            rt.block_on(async move {
                // Handle to the current task

                loop {
                    let mut current_task: Option<tokio::task::JoinHandle<_>> = None;
                    let maybe_job = match execution_plan.next() {
                        Some(ExecutionItem::Execute(executable)) => {
                            Job::new(&location, &executable)
                        }
                        Some(output) => {
                            if line_sender.send(CommandExecInfo::Output(output)).is_err() {
                                error!("Couldn't send output line, channel maybe closed");
                            };
                            continue;
                        }
                        _ => {
                            info!("End of the execution plan");
                            tokio::select! {
                                _ = &mut stop_receiver => break,
                            }
                        }
                    };
                    let maybe_command = maybe_job.as_ref().map(|j| {
                        let mut command = Command::from(j.get_command());
                        command.stdin(Stdio::null()).stderr(Stdio::piped()).stdout(
                            if j.need_stdout {
                                Stdio::piped()
                            } else {
                                Stdio::null()
                            },
                        );
                        command
                    });
                    let mut job = if maybe_job.is_some() {
                        maybe_job.unwrap()
                    } else {
                        continue;
                    };
                    let with_stdout = job.need_stdout;
                    let mut command = if maybe_command.is_some() {
                        maybe_command.unwrap()
                    } else {
                        continue;
                    };

                    // wait for the next task
                    if let Some(old) = current_task.take() {
                        old.abort();
                    }

                    let file_path = match job.write_file() {
                        Err(e) => {
                            let response =
                                CommandExecInfo::Error(format!("failed to write file: {}", e));
                            match line_sender.send(response) {
                                Err(_) => break,
                                _ => continue,
                            }
                        }
                        Ok(f) => f,
                    };
                    let path_str = String::from(file_path.clone().to_string_lossy());
                    command.arg(path_str);
                    let child = match start_task(&mut command) {
                        Err(e) => {
                            let response = CommandExecInfo::Error(format!(
                                "failed to start task: {} job: {}",
                                e, job
                            ));
                            match line_sender.send(response) {
                                Err(_) => break,
                                _ => continue,
                            }
                        }
                        Ok(child) => child,
                    };

                    if line_sender.send(CommandExecInfo::Start).is_err() {
                        error!("Couldn't send start message");
                    };

                    current_task = Some(tokio::spawn(execute_task(
                        child,
                        with_stdout,
                        line_sender.clone(),
                    )));

                    // Wait for the current task to finish
                    let response = match task_result(&mut current_task).await {
                        Err(e) => CommandExecInfo::Error(format!("failed to execute task: {}", e)),
                        Ok(status) => CommandExecInfo::End { status },
                    };

                    if line_sender.send(response).is_err() {
                        break;
                    }

                    if !keep_files && job.remove_file().is_err() {
                        error!("Couldn't remove the job file");
                        break;
                    }
                }
            })
        });

        Ok(Self {
            line_receiver,
            stop_sender,
            thread,
        })
    }

    pub fn die(self) -> Result<()> {
        debug!("received kill order");
        let _ = self.stop_sender.send(());
        self.thread.join().unwrap();
        debug!("the executor killed");
        Ok(())
    }
}

async fn task_result(
    task: &mut Option<JoinHandle<Result<Option<ExitStatus>>>>,
) -> Result<Option<ExitStatus>> {
    match task {
        Some(handle) => handle.await.unwrap(),
        None => match AlwaysPending.await {},
    }
}

/// A future that will never resolve
struct AlwaysPending;

impl std::future::Future for AlwaysPending {
    type Output = std::convert::Infallible;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Pending
    }
}

/// Start the given task/command
fn start_task(command: &mut Command) -> std::io::Result<Child> {
    command.kill_on_drop(true).spawn()
}

/// Send all lines in the process' output
async fn execute_task(
    mut child: Child,
    with_stdout: bool,
    line_sender: LineSender,
) -> Result<Option<ExitStatus>> {
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| anyhow!("child missing stderr"))?;

    let stderr_sender = line_sender.clone();
    let stderr = stream_consumer(stderr, CommandStream::StdErr, stderr_sender);

    let stdout = if with_stdout {
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow!("child missing stdout"))?;
        let stdout_sender = line_sender.clone();
        Some(stream_consumer(
            stdout,
            CommandStream::StdOut,
            stdout_sender,
        ))
    } else {
        None
    };

    // either we wait on both stdout and stderr concurrently, or just stderr.
    if let Some(stdout) = stdout {
        tokio::try_join!(stdout, stderr)?;
    } else {
        stderr.await?;
    }

    let status = match child.wait().await {
        Err(_) => None,
        Ok(status) => Some(status),
    };

    Ok(status)
}

/// Send all lines in the given stream to the sender.
async fn stream_consumer(
    stream: impl AsyncRead + Unpin,
    origin: CommandStream,
    line_sender: LineSender,
) -> Result<()> {
    let mut lines = BufReader::new(stream).lines();

    while let Some(line) = lines.next_line().await? {
        let response = CommandExecInfo::Line(CommandOutputLine {
            content: line,
            origin,
        });
        if line_sender.send(response).is_err() {
            return Err(anyhow!("channel closed"));
        }
    }

    Ok(())
}
