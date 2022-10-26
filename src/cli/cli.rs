use anyhow::bail;
use std::io::{stdout, BufWriter};
use {
    crate::*,
    clap::Parser,
    crossterm::{
        self, cursor,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
        QueueableCommand,
    },
    std::{fs, io::Write},
    termimad::EventSource,
};

use crate::cli::args::Args;
use crate::executor::execution_plan::{ExecutionItem, ExecutionPlan};
use crate::executor::job::Job;
use crate::executor::job_location::JobLocation;
use crate::executor::job_ref::JobRef;
use crate::view::View;

/// the type used by all GUI writing functions
///
/// Right now we use stderr, which has the advantage of letting
/// us output something if we want (for a calling process) but
/// as I'm not sure I'll even have something to output, I may
/// switch to stdout which would allow buffering.
//pub type W = std::io::Stderr;
pub type W = BufWriter<std::io::Stdout>;

/// return the writer used by the application
pub fn writer() -> W {
    //std::io::stderr()
    BufWriter::new(stdout())
}

pub fn run() -> anyhow::Result<Option<JobRef>> {
    let mut args: Args = Args::parse();
    args.fix()?;
    info!("args: {:#?}", &args);

    let location = JobLocation::new(&args)?;
    info!("mission location: {:#?}", &location);

    if location.path_to_md.is_none() {
        bail!("markdown file was not found");
    }

    let md_path = location.path_to_md.as_ref().unwrap();
    let file_content = fs::read_to_string(md_path)?;
    let text = minimad::clean::lines(&file_content);
    let mut execution_plan = ExecutionPlan::from_md_lines(text.into_iter());

    let mut view = View::new();
    let mut w = writer();

    // w.queue(EnterAlternateScreen)?;
    w.queue(cursor::Hide)?;

    let event_source = EventSource::new()?;
    let mut result = Ok(None);

    loop {
        let job = match execution_plan.next() {
            Some(ExecutionItem::Execute(executable)) => Job::new(&location, &executable),
            Some(ExecutionItem::Output(line)) => {
                view.write_on(&mut w, line)?;
                continue;
            }
            _ => {
                info!("End of the execution plan");
                break;
            }
        };
        w.flush()?;
        let r = job.map(|j| app::run(&mut w, &mut view, j, &event_source));
        match r {
            Some(Ok(v)) => {
                result = Ok(v);
                continue;
            }
            Some(Err(e)) => {
                result = Err(e);
                continue;
            }
            _ => continue,
        }
    }

    w.queue(cursor::Show)?;
    // w.queue(LeaveAlternateScreen)?;
    w.flush()?;
    result
}
