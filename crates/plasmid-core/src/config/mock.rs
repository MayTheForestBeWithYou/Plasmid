use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{config::error::ConfigError, loader::ConfigReader};

#[derive(Default)]
pub struct MockConfigReader {
    pub data: HashMap<PathBuf, String>,
}

impl MockConfigReader {
    #[must_use]
    pub fn with_file(mut self, path: &str, content: &str) -> Self {
        self.data.insert(PathBuf::from(path), content.to_string());
        self
    }
}

impl ConfigReader for MockConfigReader {
    fn read_file(&self, path: &Path) -> Result<Option<String>, ConfigError> {
        self.data
            .get(path)
            .map_or(Ok(None), |content| Ok(Some(content.clone())))
    }
}
