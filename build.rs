// This file is executed during compilation.
// It builds shell completions and man pages.

use std::path::PathBuf;
use {
    clap::CommandFactory,
    clap_complete::{Generator, Shell},
    std::{env, ffi::OsStr},
};

include!("src/cli/args.rs");

fn write_completions_file<G: Generator + Copy, P: AsRef<OsStr>>(generator: G, out_dir: P) {
    let mut args = Args::command();
    for name in &["gem"] {
        clap_complete::generate_to(generator, &mut args, name.to_string(), &out_dir)
            .expect("clap complete generation failed");
    }
}

fn write_man_file<P: AsRef<OsStr>>(out_dir: P) -> std::io::Result<()> {
    let args = Args::command();
    let man = clap_mangen::Man::new(args);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(PathBuf::from(out_dir.as_ref()).join("head.1"), buffer)
}

/// write the shell completion scripts and man pages which will be added to
/// the release archive
fn build() {
    let out_dir = env::var_os("OUT_DIR").expect("out dir not set");
    write_completions_file(Shell::Bash, &out_dir);
    write_completions_file(Shell::Elvish, &out_dir);
    write_completions_file(Shell::Fish, &out_dir);
    write_completions_file(Shell::PowerShell, &out_dir);
    write_completions_file(Shell::Zsh, &out_dir);

    let _ = write_man_file(&out_dir);

    eprintln!("completion scripts and manpage generated in {:?}", out_dir);
}

fn main() {
    build();
}
