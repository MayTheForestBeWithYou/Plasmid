use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to serialize default config: {0}")]
    Serialization(#[from] toml::ser::Error),

    #[error("Failed to deserialize configuration: {0}")]
    Deserialization(#[from] toml::de::Error),

    #[error("Failed to parse TOML file at '{path}': {source}")]
    TomlParseError {
        path: PathBuf,
        source: toml::de::Error,
    },
}
