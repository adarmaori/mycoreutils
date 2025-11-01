use clap::Parser;
use std::{
    ffi::OsString,
    fs::File,
    io::{self, BufRead, BufReader, Read},
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
                let mut line = String::new();
                let mut counter = 1;
                while let Ok(len) = buf_reader.read_line(&mut line) {
                    dbg!(len);
                    if len > 1 {
                        stdout.write(&format!("   {}  ", counter).as_bytes())?;
                        stdout.write(&line.as_bytes())?;
                        counter += 1;
                    } else if len == 1 {
                        writeln!(stdout, "")?;
                    } else {
                        break;
                    }
                    line.clear();
                }
            } else {
                io::copy(&mut file, stdout)?;
            }
        }
        Ok(ExitCode::from(0))
    }
}
