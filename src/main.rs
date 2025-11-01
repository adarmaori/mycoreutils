use std::{
    io::{self, BufReader},
    process::ExitCode,
};

use clap::{Parser, Subcommand};

use crate::commands::{
    Command, NoArgs,
    cat::{Cat, CatArgs},
    echo::{Echo, EchoArgs},
    head::{Head, HeadArgs},
    seq::{Seq, SeqArgs},
    yes::Yes,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Echo(EchoArgs),
    Cat(CatArgs),
    Seq(SeqArgs),
    Yes,
    Head(HeadArgs),
}

mod commands;

fn main() {
    let cli = Cli::parse();
    let _ = if let Some(command) = cli.command {
        let sid = io::stdin().lock();
        let sout = io::stdout().lock();
        let mut serr = io::stderr().lock();

        let mut reader = BufReader::new(sid);
        // Do not wrap stdout in BufWriter to avoid buffering interactive output
        let mut writer = sout;

        match command {
            Commands::Echo(args) => Echo::run(&mut reader, &mut writer, &mut serr, args),
            Commands::Cat(args) => Cat::run(&mut reader, &mut writer, &mut serr, args),
            Commands::Seq(args) => Seq::run(&mut reader, &mut writer, &mut serr, args),
            Commands::Head(args) => Head::run(&mut reader, &mut writer, &mut serr, args),
            Commands::Yes => Yes::run(&mut reader, &mut writer, &mut serr, NoArgs),
        }
    } else {
        Ok(ExitCode::from(0))
    };
}
