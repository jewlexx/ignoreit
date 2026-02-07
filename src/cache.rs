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

use crate::{
    api::{Gitignore, Gitignores},
    cache::update::LastUpdate,
};

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
        if !path.try_exists()? {
            std::fs::File::create_new(&path)?;
        }

        let db_url = Url::from_file_path(&path).expect("valid file path url");
        let opts = SqliteConnectOptions::from_url(&db_url)?;
        let pool = SqlitePool::connect_with(opts).await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self {
            cache_dir: cache_dir.to_owned(),
            pool,
        })
    }

    pub async fn update(&self, force: bool) -> anyhow::Result<()> {
        let last_update = LastUpdate::get(self.cache_dir());

        match last_update {
            // todo: handle update time
            Ok(last_update) => Self::update_inner(&self, false).await,
            Err(error) => match error {
                update::Error::MissingLastUpdate => Self::update_inner(self, true).await,
                error => Err(error)?,
            },
        }
    }

    async fn update_inner(&self, initial: bool) -> anyhow::Result<()> {
        let raw_response = reqwest::get(ALL_GITIGNORES).await?.text().await?;
        let response: Gitignores = serde_json::from_str(&raw_response)?;
        let update_data = LastUpdate::from_data(&response);
        update_data.save(self.cache_dir())?;

        let mut txn = self.pool.begin().await?;
        for row in response.values() {
            let query = if initial {
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
            } else {
                sqlx::query!(
                    r#"
                    UPDATE gitignores
                    SET contents = ?2, file_name = ?3, name = ?4
                    WHERE key = ?1
                    "#,
                    row.key,
                    row.contents,
                    row.file_name,
                    row.name,
                )
            };

            query.execute(&mut *txn).await?;
        }

        txn.commit().await?;

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
    // pub fn get_template(&self, name: &TemplatePath) -> anyhow::Result<Vec<u8>> {
    //     let path = self.db_path().join(&name.capped);

    //     if !path.exists() {
    //         Err(anyhow::anyhow!("Template not found"))
    //     } else {
    //         let mut file = fs::File::open(path).with_context(|| "Failed to open template file")?;
    //         let mut bytes = Vec::new();
    //         file.read_to_end(&mut bytes)?;

    //         Ok(bytes)
    //     }
    // }

    // todo: maybe stream this into tui??
    /// List all of the templates in the cache
    pub async fn list_templates(&self) -> anyhow::Result<Vec<String>> {
        let mut conn = self.pool.acquire().await?;
        let query = sqlx::query_as!(
            Gitignore,
            r#"
SELECT * FROM gitignores;
        "#
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(query.iter().map(|row| row.name.clone()).collect())
    }
}
