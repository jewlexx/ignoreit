use std::{
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LastUpdate {
    pub time: SystemTime,
    pub hash: u64,
}

impl LastUpdate {
    pub fn from_data(data: impl AsRef<str>) -> Self {
        let data = data.as_ref();
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let hash = hasher.finish();

        Self {
            time: SystemTime::now(),
            hash,
        }
    }

    pub fn file_name() -> PathBuf {
        "update.json".into()
    }

    pub fn get(parent_dir: impl AsRef<Path>) -> Result<Self, Error> {
        let update_path = parent_dir.as_ref().join(Self::file_name());
        if !update_path.try_exists()? {
            return Err(Error::MissingLastUpdate);
        }

        let file = File::open(update_path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn save(self, parent_dir: impl AsRef<Path>) -> Result<(), Error> {
        let update_path = parent_dir.as_ref().join(Self::file_name());
        let mut file = File::create(update_path)?;

        serde_json::to_writer(&mut file, &self)?;

        Ok(())
    }
}
