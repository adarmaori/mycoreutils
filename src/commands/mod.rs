pub mod cat;
pub mod echo;
pub mod head;
pub mod seq;
pub mod yes;

#[derive(Default)]
pub struct NoArgs;

pub trait Command {
    type Args;
    fn run<R: std::io::Read, W: std::io::Write, E: std::io::Write>(
        stdin: &mut R,
        stdout: &mut W,
        stderr: &mut E,
        args: Self::Args,
    ) -> std::io::Result<std::process::ExitCode>;
}
