mod app;
mod cli;
mod executor;
mod fenced_attributes;
mod view;

#[macro_use]
extern crate log;

use crate::cli::args::Args;
use anyhow;
use clap::Parser;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use termimad::*;

fn main() -> anyhow::Result<()> {
    let mut args: Args = Args::parse();
    args.fix()?;
    if args.verbose {
        info!("args: {:#?}", &args);
    }

    let logfile = if args.log_file.is_some() {
        Some(
            FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(
                    "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} - {l} - {m}\n",
                )))
                .build("log/output.log")?,
        )
    } else {
        None
    };

    let config_builder = Config::builder();
    let log_level = if args.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Error
    };
    let config = if logfile.is_some() {
        config_builder
            .appender(Appender::builder().build("logfile", Box::new(logfile.unwrap())))
            .build(Root::builder().appender("logfile").build(log_level))?
    } else {
        config_builder.build(Root::builder().build(log_level))?
    };

    log4rs::init_config(config)?;
    cli::cli::run(&args)?;
    info!("bye");
    Ok(())
}
