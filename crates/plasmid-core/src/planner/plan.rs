use crate::planner::operation::Operation;

#[derive(Debug, Default)]
pub struct Plan {
    pub operations: Vec<Operation>,
}

impl Plan {
    pub fn add(&mut self, op: Operation) {
        self.operations.push(op);
    }
}
