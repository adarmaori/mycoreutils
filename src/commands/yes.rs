use crate::commands::{Command, NoArgs};
use std::{process::ExitCode, write};

pub struct Yes;
impl Command for Yes {
    type Args = NoArgs;
    fn run<R: std::io::Read, W: std::io::Write, E: std::io::Write>(
        _stdin: &mut R,
        stdout: &mut W,
        _stderr: &mut E,
        _args: Self::Args,
    ) -> std::io::Result<std::process::ExitCode> {
        loop {
            match write!(stdout, "y\n") {
                Ok(_) => {}
                Err(_) => break,
            }
        }
        Ok(ExitCode::from(1))
    }
}
