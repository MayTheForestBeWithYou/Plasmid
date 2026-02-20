use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PlasmidConfig {
    pub ignore: Vec<String>,
    pub packages: Vec<PackageSpec>,
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct PackageSpec {
    pub name: String,
    pub manager: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct Profile {
    pub packages: Vec<PackageSpec>,
    pub variables: HashMap<String, String>,
}

impl Default for PlasmidConfig {
    fn default() -> Self {
        Self {
            ignore: vec![".gitignore".to_string()],
            packages: Vec::default(),
            profiles: HashMap::default(),
        }
    }
}
