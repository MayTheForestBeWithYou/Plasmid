use std::io::Result as ioResult;
use std::process::{Command, Output};

pub trait CommandRunner {
    /// Run command in the specified commandrunner
    ///
    /// # Errors
    /// This method returns an error if:
    /// - The underlying commandrunner fails to execute the command
    fn run(&self, cmd: &mut Command) -> ioResult<Output>;
}

pub struct RealCommandRunner;

impl CommandRunner for RealCommandRunner {
    fn run(&self, cmd: &mut Command) -> ioResult<Output> {
        cmd.output()
    }
}
