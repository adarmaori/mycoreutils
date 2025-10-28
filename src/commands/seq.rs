use std::process::ExitCode;

use clap::Parser;

use crate::commands::Command;

pub struct Seq;

#[derive(Parser)]
pub struct SeqArgs {
    start: u64,
    end: u64,
}

impl Command for Seq {
    type Args = SeqArgs;

    fn run<R: std::io::Read, W: std::io::Write, E: std::io::Write>(
        _stdin: &mut R,
        stdout: &mut W,
        _stderr: &mut E,
        args: Self::Args,
    ) -> std::io::Result<std::process::ExitCode> {
        for i in args.start..=args.end {
            stdout.write_all(&format!("{i}\n").into_bytes())?;
        }
        Ok(ExitCode::from(0))
    }
}
