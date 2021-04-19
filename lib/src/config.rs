//! Configuration logic which can be used in both CLI and Server contexts
//! For serializaing, storing, and parsing the `~/.config/atomic/config.toml` file

use crate::errors::AtomicResult;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// A set of options that are shared between CLI and Server contexts
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// URL of Companion Atomic Server, where data is written to by default.
    pub server: String,
    /// The current Agent (user) URL. Usually lives on the server, but not necessarily so.
    pub agent: String,
    /// Private key for the Agent, which is used to sign commits.
    pub private_key: String,
}

/// Returns the default path for the config file: `~/.config/atomic`
pub fn default_config_dir_path() -> AtomicResult<PathBuf> {
    Ok(dirs::home_dir()
        .ok_or("Could not open home dir")?
        .join(".config/atomic"))
}

/// Returns the default path for the config file: `~/.config/atomic/config.toml`
pub fn default_config_file_path() -> AtomicResult<PathBuf> {
    let mut default_dir = default_config_dir_path()?;
    default_dir.push("config.toml");
    Ok(default_dir)
}

/// Reads config file from a specified path
pub fn read_config(path: &Path) -> AtomicResult<Config> {
    let config_string = std::fs::read_to_string(path)
        .map_err(|e| format!("Error reading config from {:?}. {}", path, e))?;
    let config: Config = toml::from_str(&config_string)
        .map_err(|e| format!("Could not parse toml in config file {:?}. {}", path, e))?;
    Ok(config)
}

/// Writes config file from a specified path
/// Overwrites any existing config
pub fn write_config(path: &Path, config: Config) -> AtomicResult<String> {
    let out =
        toml::to_string_pretty(&config).map_err(|e| format!("Error serializing config. {}", e))?;
    std::fs::write(path, out.clone())
        .map_err(|e| format!("Error writing config to {:?}. {}", path, e))?;
    Ok(out)
}
