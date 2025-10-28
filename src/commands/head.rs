use std::{
    io::{BufRead, BufReader},
    process::ExitCode,
};

use clap::Parser;

use crate::commands::Command;

pub struct Head;

#[derive(Parser)]
pub struct HeadArgs {
    #[arg(short = 'n')]
    n: u64,
}

impl Command for Head {
    type Args = HeadArgs;

    fn run<R: std::io::Read, W: std::io::Write, E: std::io::Write>(
        stdin: &mut R,
        stdout: &mut W,
        _stderr: &mut E,
        args: Self::Args,
    ) -> std::io::Result<std::process::ExitCode> {
        let mut counter = 0;
        let mut line_buf: Vec<u8> = Vec::new();
        let mut buf_reader = BufReader::new(stdin);
        loop {
            line_buf.clear();
            buf_reader.read_until(b'\n', &mut line_buf)?;
            stdout.write(&line_buf)?;
            counter += 1;
            if counter == args.n {
                break;
            }
        }
        Ok(ExitCode::from(0))
    }
}
