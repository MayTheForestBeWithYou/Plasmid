use std::path::Path;

use plasmid_fs::{filesystem::operation::FileSystemOps, walker::walkdir::FileMapping};

use crate::planner::{operation::Operation, plan::Plan};

pub struct Planner<F: FileSystemOps> {
    fs: F,
}

impl<F: FileSystemOps> Planner<F> {
    pub const fn new(fs: F) -> Self {
        Self { fs }
    }

    pub fn plan(&self, mappings: &Vec<FileMapping>) -> Plan {
        let mut plan = Plan::default();

        for mapping in mappings {
            let op = self.evaluate(&mapping.source, &mapping.dest);
            plan.add(op);
        }

        plan
    }

    fn evaluate(&self, src: &Path, dest: &Path) -> Operation {
        let exists = self.fs.exists(dest);

        if !exists && self.fs.is_file(src) {
            return Operation::Link {
                src: src.to_path_buf(),
                dest: dest.to_path_buf(),
            };
        } else if !exists && self.fs.is_dir(src) {
            return Operation::MkDir {
                path: dest.to_path_buf(),
            };
        }

        Operation::NoOp
    }
}
