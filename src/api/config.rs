use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_username: String,
    pub api_key: String,
}

impl Config {
    pub fn config_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not determine home directory")?;
        Ok(home.join(".config").join("cb"))
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn save(&self) -> Result<()> {
        let dir = Self::config_dir()?;
        fs::create_dir_all(&dir).context("Failed to create config directory")?;
        let toml = toml::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(Self::config_path()?, toml).context("Failed to write config file")?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        let content = fs::read_to_string(&path).with_context(|| {
            format!(
                "No config found at {}. Run `cb login` first.",
                path.display()
            )
        })?;
        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    }

    /// Parse the account name from the API username (e.g. "sectormobile/samkrishna" -> "sectormobile")
    pub fn account(&self) -> &str {
        self.api_username
            .split('/')
            .next()
            .unwrap_or(&self.api_username)
    }

    /// Parse the username from the API username (e.g. "sectormobile/samkrishna" -> "samkrishna")
    pub fn username(&self) -> &str {
        self.api_username
            .split('/')
            .nth(1)
            .unwrap_or(&self.api_username)
    }
}
