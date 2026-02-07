use std::{
    fs::File,
    path::{Path, PathBuf},
    time::SystemTime,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Last update time was not present")]
    MissingLastUpdate,
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),
    #[error("ser/de json: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastUpdate(pub SystemTime);

impl LastUpdate {
    pub fn now() -> Self {
        Self(SystemTime::now())
    }

    pub fn file_name() -> PathBuf {
        "update.json".into()
    }

    pub fn get(parent_dir: impl AsRef<Path>) -> Result<Self, Error> {
        let update_path = parent_dir.as_ref().join(Self::file_name());
        if !update_path.try_exists()? {
            return Err(Error::MissingLastUpdate);
        }

        let file = File::create(update_path)?;
        Ok(serde_json::from_reader(file)?)
    }
}
