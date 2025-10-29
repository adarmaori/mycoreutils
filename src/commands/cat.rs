use clap::Parser;
use std::{fs::File, io, path::PathBuf, process::ExitCode};

use crate::commands::Command;

#[derive(Parser)]
pub struct CatArgs {
    files: Vec<PathBuf>,
}
pub struct Cat;
impl Command for Cat {
    type Args = CatArgs;

    fn run<R: std::io::Read, W: std::io::Write, E: std::io::Write>(
        _stdin: &mut R,
        stdout: &mut W,
        _stderr: &mut E,
        args: Self::Args,
    ) -> std::io::Result<std::process::ExitCode> {
        for source in args.files {
            let mut file = File::open(source)?;
            io::copy(&mut file, stdout)?;
        }
        Ok(ExitCode::from(0))
    }
}
