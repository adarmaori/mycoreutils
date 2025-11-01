use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
    process::ExitCode,
};

use crate::commands::Command;

#[derive(Parser)]
pub struct CatArgs {
    files: Vec<PathBuf>,

    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank: bool,
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
            if args.number_nonblank {
                let mut buf_reader = BufReader::new(file);
                let mut buf: Vec<u8> = Vec::new();
                let mut counter: usize = 1;
                loop {
                    buf.clear();
                    let n = buf_reader.read_until(b'\n', &mut buf)?;
                    if n == 0 {
                        break;
                    }

                    // Determine if the line (excluding trailing newlines) is blank
                    let is_blank = {
                        let mut end = buf.len();
                        while end > 0 && (buf[end - 1] == b'\n' || buf[end - 1] == b'\r') {
                            end -= 1;
                        }
                        end == 0
                    };

                    if !is_blank {
                        // Match `cat -b` style: right-align to width 6, then a tab
                        write!(stdout, "{:>6}\t", counter)?;
                        counter += 1;
                    }
                    stdout.write_all(&buf)?;
                }
            } else {
                io::copy(&mut file, stdout)?;
            }
        }
        Ok(ExitCode::from(0))
    }
}
