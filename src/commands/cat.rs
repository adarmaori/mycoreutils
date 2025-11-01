use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Write, stdin},
    path::{Path, PathBuf},
    process::ExitCode,
};

use crate::commands::Command;

#[derive(Parser, Clone)]
pub struct CatArgs {
    files: Vec<PathBuf>,

    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank: bool,
}
pub struct Cat;

fn handle_source(
    source: &mut impl Read,
    stdout: &mut impl Write,
    args: CatArgs,
) -> std::io::Result<()> {
    if args.number_nonblank {
        let mut buf_reader = BufReader::new(source);
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
        io::copy(source, stdout)?;
    }
    Ok(())
}

impl Command for Cat {
    type Args = CatArgs;

    fn run<R: std::io::Read, W: std::io::Write, E: std::io::Write>(
        stdin: &mut R,
        stdout: &mut W,
        _stderr: &mut E,
        args: Self::Args,
    ) -> std::io::Result<std::process::ExitCode> {
        for source in &args.files {
            if source == Path::new("-") {
                handle_source(stdin, stdout, args.clone())?;
            } else {
                let mut file = File::open(source)?;
                handle_source(&mut file, stdout, args.clone())?;
            };
        }
        stdout.flush()?;
        Ok(ExitCode::from(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::Command;
    use std::{
        fs::{self, File},
        io::{self, BufReader, BufWriter, Write},
        path::PathBuf,
        process::ExitCode,
    };

    fn write_temp_file(contents: &[u8]) -> io::Result<PathBuf> {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "mycoreutils_cat_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        path.push(unique);
        let mut f = File::create(&path)?;
        f.write_all(contents)?;
        Ok(path)
    }

    fn run_with_args(args: CatArgs, stdin: &Option<Vec<u8>>) -> io::Result<(ExitCode, Vec<u8>)> {
        let stdout: Vec<u8> = Vec::new();
        let mut stdout_buf = BufWriter::new(stdout);
        let empty = Vec::<u8>::new();
        let mut stdin_buf = if let Some(stdin1) = stdin {
            BufReader::new(stdin1.as_slice())
        } else {
            BufReader::new(&empty[..])
        };
        let mut stderr: Vec<u8> = Vec::new();
        let res: ExitCode = Cat::run(&mut stdin_buf, &mut stdout_buf, &mut stderr, args)?;
        stdout_buf.flush()?;
        Ok((res, Vec::from_iter(stdout_buf.into_inner()?)))
    }

    #[test]
    fn normal() {
        let data = b"hello\0world\nnext line\n";
        let path = write_temp_file(data).expect("temp file");
        let args = CatArgs {
            files: vec![path.clone()],
            number_nonblank: false,
        };
        let (code, out) = run_with_args(args, &None).expect("run");
        assert_eq!(code, ExitCode::from(0));
        assert_eq!(out, data);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn number_nonblank_mixes_blank_and_nonblank() {
        let data = b"a\n\nb\n\n\nc\n";
        let path = write_temp_file(data).expect("temp file");
        let args = CatArgs {
            files: vec![path.clone()],
            number_nonblank: true,
        };
        let (code, out) = run_with_args(args, &None).expect("run");
        assert_eq!(code, ExitCode::from(0));

        let expected = b"     1\ta\n\n     2\tb\n\n\n     3\tc\n";
        assert_eq!(out, expected);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn number_nonblank_no_trailing_newline() {
        // a, blank, b (no trailing newline)
        let data = b"a\n\nb";
        let path = write_temp_file(data).expect("temp file");
        let args = CatArgs {
            files: vec![path.clone()],
            number_nonblank: true,
        };
        let (code, out) = run_with_args(args, &None).expect("run");
        assert_eq!(code, ExitCode::from(0));

        let expected = b"     1\ta\n\n     2\tb";
        assert_eq!(out, expected);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn number_nonblank_crlf_blank_detection() {
        // blank CRLF, blank CRLF, x CRLF
        let data = b"\r\n\r\nx\r\n";
        let path = write_temp_file(data).expect("temp file");
        let args = CatArgs {
            files: vec![path.clone()],
            number_nonblank: true,
        };
        let (code, out) = run_with_args(args, &None).expect("run");
        assert_eq!(code, ExitCode::from(0));

        let expected = b"\r\n\r\n     1\tx\r\n";
        assert_eq!(out, expected);
        let _ = fs::remove_file(path);
    }
}
