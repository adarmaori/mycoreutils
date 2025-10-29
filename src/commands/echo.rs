use std::{ffi::OsString, process::ExitCode};

use clap::Parser;

use crate::commands::Command;

pub struct Echo;

#[derive(Parser)]
pub struct EchoArgs {
    strings: Vec<OsString>,
    #[arg(short = 'n')]
    new_line: bool,
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
        if !args.new_line {
            stdout.write_all(b"\n")?;
        }
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
        io::{self, BufReader, BufWriter, Write},
        os::unix::ffi::OsStrExt,
        process::ExitCode,
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

    fn run_with_args(args: EchoArgs) -> io::Result<(ExitCode, Vec<u8>)> {
        let stdout: Vec<u8> = Vec::new();
        let mut stdout_buf = BufWriter::new(stdout);
        let stdin: &[u8] = b""; // Not reading from stdin
        let mut stdin = BufReader::new(stdin);
        let mut stderr: Vec<u8> = Vec::new();
        let res: ExitCode = Echo::run(&mut stdin, &mut stdout_buf, &mut stderr, args)?;
        stdout_buf.flush()?;
        Ok((res, Vec::from_iter(stdout_buf.into_inner()?)))
    }

    #[test]
    fn normal() {
        let strings = random_echo_args(10, 10);
        let args = EchoArgs {
            strings: strings.clone(),
            new_line: false,
        };
        let result = run_with_args(args);
        assert!(result.is_ok());

        // NOTE: This should always happen thanks to the previous assertion
        if let Ok((code, printout)) = result {
            assert_eq!(code, ExitCode::from(0));
            assert_eq!(printout, chain_args(strings).as_bytes())
        }
    }
    #[test]
    fn with_n_flag() {
        let strings = random_echo_args(10, 10);
        let args = EchoArgs {
            strings: strings.clone(),
            new_line: true,
        };
        let result = run_with_args(args);
        assert!(result.is_ok());

        // NOTE: This should always happen thanks to the previous assertion
        if let Ok((code, printout)) = result {
            let chained = chain_args(strings);
            let chained = chained.as_encoded_bytes();
            assert_eq!(code, ExitCode::from(0));
            assert_eq!(printout, chained[..chained.len() - 1]);
        }
    }
}
