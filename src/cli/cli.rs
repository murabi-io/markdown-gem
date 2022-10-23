use anyhow::bail;
use crossterm::style::Color::{AnsiValue, Magenta, Yellow};
use crossterm::style::Print;
use std::fmt::{Display, Formatter};
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
use crate::executor::command_output::CommandExecInfo::Line;
use crate::executor::execution_plan::{ExecutionItem, ExecutionPlan};
use crate::executor::job::Job;
use crate::executor::job_ref::JobRef;
use crate::executor::mission::Mission;
use crate::executor::mission_location::MissionLocation;
use crate::view::View;
use crate::Line::Normal;

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

pub fn run() -> anyhow::Result<()> {
    let mut args: Args = Args::parse();
    args.fix()?;
    info!("args: {:#?}", &args);

    let location = MissionLocation::new(&args)?;
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
    let mut result = Ok(());

    loop {
        let job = match execution_plan.next() {
            Some(ExecutionItem::Execute(executable)) => Job::from_executable(&executable),
            Some(ExecutionItem::Output(line)) => {
                // let fmt_line = FmtLine::from(line, &render_skin);
                // w.queue(Print(FmtText {
                //     lines: vec![fmt_line],
                //     skin: &render_skin,
                //     width: Some(width as usize),
                // }))?;
                // w.flush()?;
                view.write_on(&mut w, line)?;
                continue;
            }
            _ => break,
        };
        // let r = job.and_then(|j| app::run(&mut w, j, &event_source));
        // match r {
        //     Ok(Some(job_ref)) => {
        //         next_job = job_ref;
        //     }
        //     Ok(None) => {
        //         break;
        //     }
        //     Err(e) => {
        //         result = Err(e);
        //         break;
        //     }
        // }
    }

    w.queue(cursor::Show)?;
    // w.queue(LeaveAlternateScreen)?;
    w.flush()?;
    result
}
