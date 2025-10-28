use std::{ffi::OsString, process::ExitCode};

use clap::Parser;

use crate::commands::Command;

pub struct Echo;

#[derive(Parser)]
pub struct EchoArgs {
    strings: Vec<OsString>,
}

impl Command for Echo {
    type Args = EchoArgs;

    fn run<R: std::io::Read, W: std::io::Write, E: std::io::Write>(
        _stdin: &mut R,
        stdout: &mut W,
        _stderr: &mut E,
        args: Self::Args,
    ) -> std::io::Result<std::process::ExitCode> {
        let mut first = true;
        for string in args.strings {
            if !first {
                stdout.write(b" ")?;
            } else {
                first = false;
            }
            stdout.write(&string.into_encoded_bytes())?;
        }
        stdout.write(b"\n")?;
        Ok(ExitCode::from(0))
    }
}
