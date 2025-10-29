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
                stdout.write_all(b" ")?;
            } else {
                first = false;
            }
            stdout.write_all(&string.into_encoded_bytes())?;
        }
        stdout.write_all(b"\n")?;
        Ok(ExitCode::from(0))
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::{
        Command,
        echo::{Echo, EchoArgs},
    };
    use rand::Rng;
    use std::{
        ffi::{OsStr, OsString},
        io::{BufReader, BufWriter, Write},
        os::unix::ffi::OsStrExt,
    };

    fn random_echo_args(count: usize, max_len: usize) -> Vec<OsString> {
        use std::os::unix::ffi::OsStringExt;
        let mut rng = rand::rng();

        (0..count)
            .map(|_| {
                // Generate random bytes excluding '\0'
                let len = rng.random_range(1..max_len);
                let bytes: Vec<u8> = (0..len)
                    .map(|_| {
                        rng.random_range(1u8..=0x7F) // avoid 0x00
                    })
                    .collect();
                OsString::from_vec(bytes)
            })
            .collect()
    }

    fn chain_args(strings: Vec<OsString>) -> OsString {
        let seperator = OsStr::from_bytes(b" ");
        let newline = OsStr::from_bytes(b"\n");
        let mut joined = strings.join(seperator);
        joined.push(newline);
        joined
    }
    #[test]
    fn normal() {
        let stdout: Vec<u8> = Vec::new();
        let mut stdout_buf = BufWriter::new(stdout);
        let stdin: &[u8] = b""; // Not reading from stdin
        let mut stdin = BufReader::new(stdin);
        let mut stderr: Vec<u8> = Vec::new();
        let strings = random_echo_args(10, 10);
        let args = EchoArgs {
            strings: strings.clone(),
        };
        assert!(Echo::run(&mut stdin, &mut stdout_buf, &mut stderr, args).is_ok());
        assert!(stdout_buf.flush().is_ok());
        assert_eq!(
            stdout_buf.into_inner().unwrap(),
            chain_args(strings).into_encoded_bytes()
        );
    }
}
