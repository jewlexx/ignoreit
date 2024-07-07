use std::path::PathBuf;

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
        let dirs = directories::ProjectDirs::from("com", "jewlexx", "ignoreit")
            .ok_or(anyhow::anyhow!("Could not find base directories"))?;

        let path = dirs.config_dir();

        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }

        Ok(path.to_owned())
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
