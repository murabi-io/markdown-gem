use anyhow::{self};
use cli_log;
use cli_log::*;
use minimad::Line;
use termimad::*;

mod app;
mod cli;
mod executor;
mod fenced_attributes;
mod state;
mod view;

fn main() -> anyhow::Result<()> {
    init_cli_log!();
    cli::cli::run()?;
    info!("bye");
    Ok(())
}
