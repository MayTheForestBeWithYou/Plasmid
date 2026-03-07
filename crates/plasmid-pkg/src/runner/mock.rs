use crate::{error::PackageManagerError, runner::commandrunner::CommandRunner};
use std::io::Result as ioResult;
use std::{
    iter::once,
    os::windows::process::ExitStatusExt,
    process::{Command, ExitStatus, Output},
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct ExpectedCommand {
    pub expected: String,
    pub output: Output,
}

#[derive(Clone, Default)]
pub struct MockCommandRunner {
    queue: Arc<Mutex<Vec<ExpectedCommand>>>,
}

impl MockCommandRunner {
    #[must_use]
    pub fn new(queue: Vec<ExpectedCommand>) -> Self {
        Self {
            queue: Arc::new(Mutex::new(queue)),
        }
    }

    #[must_use]
    pub fn builder() -> MockCommandRunnerBuilder {
        MockCommandRunnerBuilder::default()
    }

    /// Check if queue is empty to determine if all expected commands were executed
    ///
    /// # Errors
    /// This method returns an error if:
    /// - Queue has a poisoned lock
    ///
    /// # Panics
    /// This method panics if the queue is not empty
    pub fn assert_empty(&self) -> Result<(), PackageManagerError> {
        let queue_lock = self.queue.lock();
        let queue = queue_lock.map_err(|e| PackageManagerError::LockError(format!("{e:?}")))?;
        assert!(
            queue.is_empty(),
            "Not all expected commands were executed. Remaining: {}",
            queue.len()
        );
        drop(queue);
        Ok(())
    }

    fn normalize_command(cmd: &Command) -> String {
        let program = cmd.get_program().to_string_lossy().to_string();
        let args = cmd.get_args().map(|a| a.to_string_lossy().to_string());
        once(program).chain(args).collect::<Vec<String>>().join(" ")
    }
}

impl CommandRunner for MockCommandRunner {
    fn run(&self, cmd: &mut Command) -> ioResult<Output> {
        let Ok(mut queue) = self.queue.lock() else {
            panic!("Queue could not be unpacked")
        };

        assert!(!queue.is_empty(), "Unexpected command executed: {cmd:?}");

        let next = queue.remove(0);
        let actual = Self::normalize_command(cmd);

        assert!(
            actual.contains(&next.expected),
            "Command mismatch.\nExpected: {}\nActual: {}",
            next.expected,
            actual
        );

        Ok(next.output)
    }
}

#[derive(Default)]
pub struct MockCommandRunnerBuilder {
    items: Vec<ExpectedCommand>,
}

impl MockCommandRunnerBuilder {
    pub fn expect<S: Into<String>>(self, expected: S) -> ExpectedCommandBuilder {
        ExpectedCommandBuilder {
            parent: self,
            expected: expected.into(),
            status: 0,
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }

    #[must_use]
    pub fn push(mut self, cmd: ExpectedCommand) -> Self {
        self.items.push(cmd);
        self
    }

    #[must_use]
    pub fn build(self) -> MockCommandRunner {
        MockCommandRunner::new(self.items)
    }
}

pub struct ExpectedCommandBuilder {
    parent: MockCommandRunnerBuilder,
    expected: String,
    status: u32,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl ExpectedCommandBuilder {
    #[must_use]
    pub const fn status(mut self, code: u32) -> Self {
        self.status = code;
        self
    }

    #[must_use]
    pub fn stdout<S: Into<Vec<u8>>>(mut self, out: S) -> Self {
        self.stdout = out.into();
        self
    }

    #[must_use]
    pub fn stderr<S: Into<Vec<u8>>>(mut self, err: S) -> Self {
        self.stderr = err.into();
        self
    }

    #[must_use]
    pub fn finish(mut self) -> MockCommandRunnerBuilder {
        let output = Output {
            status: ExitStatus::from_raw(self.status),
            stdout: self.stdout,
            stderr: self.stderr,
        };

        let cmd = ExpectedCommand {
            expected: self.expected,
            output,
        };

        self.parent.items.push(cmd);
        self.parent
    }
}
