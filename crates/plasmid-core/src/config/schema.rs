use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PlasmidConfig {
    ignore: Vec<String>,
    packages: Vec<PackageSpec>,
    profiles: HashMap<String, Profile>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
struct PackageSpec {
    name: String,
    manager: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
struct Profile {
    packages: Vec<PackageSpec>,
    variables: HashMap<String, String>,
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
