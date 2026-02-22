use crate::config::{error::ConfigError, schema::PlasmidConfig};
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
mod tests {
    use crate::config::mock::MockConfigReader;

    use super::*;
    use std::{error::Error, io};

    #[test]
    fn test_load_config_returns_default_config() -> Result<(), Box<dyn Error>> {
        let reader = MockConfigReader::default();
        let config = load_config(Path::new("nonexistent"), &reader)?;
        assert_eq!(PlasmidConfig::default(), config);
        Ok(())
    }

    #[test]
    fn test_load_config_returns_valid_config_from_toml() -> Result<(), Box<dyn Error>> {
        let toml_content = r#"
        # Top‑level ignore list
        ignore = [
            "target",
            "node_modules",
        ]

        # Top‑level packages array
        [[packages]]
        name = "serde"
        manager = "cargo"

        [[packages]]
        name = "ripgrep"
        manager = "brew"

        # Profiles table (HashMap<String, Profile>)
        [profiles.dev]
        # Packages inside this profile
        [[profiles.dev.packages]]
        name = "clippy"
        manager = "cargo"

        [profiles.dev.variables] 
        RUST_LOG = "debug" 
        OPT_LEVEL = "0"
        "#;

        let path = Path::new("plasmid.toml");
        let reader = MockConfigReader::default().with_file("plasmid.toml", toml_content);

        let result = load_config(path, &reader);
        assert!(
            result.is_ok(),
            "Failed to parse valid TOML: {:?}",
            result.err()
        );

        let config = result?;

        assert!(config.ignore.contains(&"target".to_string()));
        assert_eq!(config.packages[0].name, "serde");
        assert!(config.profiles.contains_key("dev"));
        assert_eq!(
            config
                .profiles
                .get("dev")
                .ok_or_else(|| io::Error::other("missing profile"))?
                .packages[0]
                .name,
            "clippy"
        );

        Ok(())
    }

    #[test]
    fn test_load_config_returns_error() {
        let reader = MockConfigReader::default().with_file("test.toml", "");
        let result = load_config(Path::new("test.toml"), &reader);
        assert!(result.is_err());

        let Err(config_error) = result else {
            panic!("Should crash on TomlParseError")
        };
        assert!(matches!(config_error, ConfigError::TomlParseError { .. }));
    }
}
