use std::path::PathBuf;

use anyhow::Context;

use crate::dirs;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub last_update: u64,
    pub first_run: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            last_update: 0,
            first_run: true,
        }
    }
}

impl Config {
    fn config_dir() -> anyhow::Result<PathBuf> {
        let path = dirs::config_dir().context("Could not find or create config directory")?;

        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        Ok(path)
    }

    fn config_file() -> anyhow::Result<PathBuf> {
        let path = Self::config_dir()?.join("config.json");

        if !path.exists() {
            std::fs::write(
                path.clone(),
                serde_json::to_string_pretty(&Self::default())?,
            )?;
        }

        Ok(path)
    }

    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_file()?;

        if !path.exists() {
            std::fs::write(
                path.clone(),
                serde_json::to_string_pretty(&Self::default())?,
            )?;
        }

        Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_file()?;

        std::fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}
