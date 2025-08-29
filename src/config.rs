use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(std::io::Error),
    #[error("Toml error: {0}")]
    TomlDeserialize(toml::de::Error),
    #[error("Toml error: {0}")]
    TomlSerialize(toml::ser::Error),
}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ConfigError> {
        let content = read_to_string(path).map_err(|e| ConfigError::Io(e))?;
        let config: Config =
            toml::from_str(&content).map_err(|e| ConfigError::TomlDeserialize(e))?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), ConfigError> {
        let content = toml::to_string_pretty(self).map_err(|e| ConfigError::TomlSerialize(e))?;
        write(path, content).map_err(|e| ConfigError::Io(e))
    }
}
