use crate::{config::schema::PlasmidConfig, error::ConfigError};
#[cfg(test)]
use std::collections::HashMap;
use std::{fs::read_to_string, path::Path};

/// Load the Plasmid config.
///
/// # Errors
/// Returns `ConfigError`:
/// - `IoError`
/// - `Serialization`
/// - `Deserialization`
/// - `TomlParseError`
pub fn load_config(path: &Path, reader: &impl ConfigReader) -> Result<PlasmidConfig, ConfigError> {
    let content_opt = reader.read_file(path)?;

    let Some(content) = content_opt else {
        return Ok(PlasmidConfig::default());
    };

    let config: PlasmidConfig =
        toml::from_str(&content).map_err(|e| ConfigError::TomlParseError {
            path: path.to_path_buf(),
            source: e,
        })?;

    Ok(config)
}

/// Defines the behavior for retrieving configuration content.
pub trait ConfigReader {
    /// Attempts to read the file at the given path.
    ///
    /// # Errors
    /// Returns:
    /// - Ok(Some(String)): File exists and was read successfully.
    /// - Ok(None): File does not exist (not an error, trigger Mirror Mode).
    /// - Err(ConfigError): File exists but could not be read (permissions, etc.).
    fn read_file(&self, path: &Path) -> Result<Option<String>, ConfigError>;
}

pub struct FileConfigReader;

impl ConfigReader for FileConfigReader {
    fn read_file(&self, path: &Path) -> Result<Option<String>, ConfigError> {
        match read_to_string(path) {
            Ok(content) => Ok(Some(content)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(ConfigError::IoError(e)),
        }
    }
}

#[cfg(test)]
pub struct MockConfigReader {
    pub files: HashMap<String, String>,
}

#[cfg(test)]
impl ConfigReader for MockConfigReader {
    fn read_file(&self, path: &Path) -> Result<Option<String>, ConfigError> {
        let key = path.to_string_lossy().to_string();

        self.files
            .get(&key)
            .map_or(Ok(None), |content| Ok(Some(content.clone())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_returns_default_config() {
        let reader = MockConfigReader {
            files: HashMap::new(),
        };
        let config = match load_config(Path::new("s"), &reader) {
            Ok(r) => r,
            Err(e) => panic!("{}", e.to_string()),
        };
        assert_eq!(PlasmidConfig::default(), config);
    }

    
}
