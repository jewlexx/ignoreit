//! Handle gitignore cache

// TODO: Refactor into cache struct

mod update;

use std::{
    fs::{self, DirEntry},
    io::Read,
    path::{Path, PathBuf},
};

const ALL_GITIGNORES: &str = "https://www.toptal.com/developers/gitignore/api/list?format=json";

use anyhow::Context;

use directories::ProjectDirs;
use reqwest::Url;
use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};

use crate::{api, cache::update::LastUpdate};

pub const DB_PATH: &str = "gitignores.db";

pub struct CacheHandler {
    cache_dir: PathBuf,
    pool: SqlitePool,
}

impl CacheHandler {
    pub async fn new() -> anyhow::Result<Self> {
        let dirs = ProjectDirs::from("dev", "cordor", "ignoreit")
            .context("project directories creation")?;
        let cache_dir = dirs.cache_dir();

        if !cache_dir.try_exists()? {
            std::fs::create_dir_all(cache_dir)?;
        }

        let path = cache_dir.join(DB_PATH);
        let db_url = Url::from_file_path(&path).expect("valid file path url");
        let opts = SqliteConnectOptions::from_url(&db_url)?;
        let pool = SqlitePool::connect_with(opts).await?;

        Ok(Self {
            cache_dir: cache_dir.to_owned(),
            pool,
        })
    }

    pub async fn update(&self, force: bool) -> anyhow::Result<()> {
        if force {
            return Self::update_inner(self).await;
        }

        let last_update = LastUpdate::get(self.cache_dir());

        match last_update {
            Ok(last_update) => todo!("handle updating after timeout or forced update"),
            Err(error) => match error {
                update::Error::MissingLastUpdate => Self::update_inner(self).await,
                error => Err(error)?,
            },
        }
    }

    async fn update_inner(&self) -> anyhow::Result<()> {
        let response: api::Gitignores = reqwest::get(ALL_GITIGNORES).await?.json().await?;

        let mut txn = self.pool.begin().await?;
        for row in response.values() {
            sqlx::query!(
                r#"
INSERT INTO gitignores ( key, contents, file_name, name )
VALUES ( ?1, ?2, ?3, ?4 )
                "#,
                row.key,
                row.contents,
                row.file_name,
                row.name,
            )
            .execute(&mut *txn)
            .await?;
        }

        Ok(())
    }

    pub fn db_path(&self) -> PathBuf {
        self.cache_dir.join(DB_PATH)
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    fn update_path(&self) -> PathBuf {
        self.cache_dir().join(LastUpdate::file_name())
    }

    /// Purge the current cache
    pub fn purge(&self) -> anyhow::Result<()> {
        fs::remove_dir_all(self.cache_dir()).context("Failed to purge cache")?;

        Ok(())
    }

    /// Get a given template by name and return it's byte representation
    pub fn get_template(&self, name: &TemplatePath) -> anyhow::Result<Vec<u8>> {
        let path = self.db_path().join(&name.capped);

        if !path.exists() {
            Err(anyhow::anyhow!("Template not found"))
        } else {
            let mut file = fs::File::open(path).with_context(|| "Failed to open template file")?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;

            Ok(bytes)
        }
    }

    /// List all of the templates in the cache
    pub fn get_template_paths(&self) -> anyhow::Result<Vec<TemplatePath>> {
        let dir: Vec<DirEntry> = fs::read_dir(self.db_path())
            .context("Failed to read cache directory")?
            .collect::<Result<_, _>>()?;

        let ignores = dir
            .iter()
            .filter(|entry| {
                entry.file_type().unwrap().is_file()
                    && entry.file_name().to_str().unwrap() != ".hash"
            })
            .map(|entry| {
                let file_name = entry.file_name();
                let capped = file_name
                    .to_str()
                    .expect("invalid utf-8 file name")
                    .to_string();

                let lower = capped.to_lowercase();

                TemplatePath { lower, capped }
            })
            .collect();

        Ok(ignores)
    }
}

#[derive(PartialEq, Eq)]
/// Structural representation of a template path
pub struct TemplatePath {
    /// Lowercase name
    pub lower: String,
    /// Cased name
    pub capped: String,
}

impl std::fmt::Display for TemplatePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.capped)
    }
}
