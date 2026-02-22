use colored::Colorize;
use plasmid_fs::filesystem::operation::FileSystemOps;

use crate::planner::{error::PlannerError, operation::Operation};

#[derive(Debug, Default)]
pub struct Plan {
    pub operations: Vec<Operation>,
}

impl Plan {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    pub fn add(&mut self, op: Operation) {
        self.operations.push(op);
    }

    pub fn print(&self) {
        if self.is_empty() {
            println!("{}", "Plan is empty. Everything looks up to date!".green());
            return;
        }

        println!("{}", "Execution Plan:".bold().underline());
        for op in &self.operations {
            op.pretty_print();
        }
    }

    /// Executes all stored operations.
    ///
    /// # Errors
    /// This method returns an error if:
    /// - The underlying fs call fails
    pub fn execute(&self, fs: &dyn FileSystemOps) -> Result<(), PlannerError> {
        for op in &self.operations {
            op.execute(fs)?;
        }
        Ok(())
    }
}
