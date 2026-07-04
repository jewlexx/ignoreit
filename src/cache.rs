//! Handle gitignore cache

// TODO: Refactor into cache struct

mod update;

use std::{
    fs::{self},
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

const ALL_GITIGNORES: &str = "https://www.toptal.com/developers/gitignore/api/list?format=json";

use anyhow::Context;

use directories::ProjectDirs;
use futures_util::StreamExt;
use indicatif::ProgressStyle;
use reqwest::Url;
use sqlx::{
    ConnectOptions, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteQueryResult},
};

use crate::{
    api::{Gitignore, Gitignores},
    cache::update::LastUpdate,
};

pub const DB_PATH: &str = "gitignores.db";
// todo: configurable update timeout
pub const UPDATE_TIMEOUT: Duration = Duration::from_hours(1);

pub struct CacheHandler {
    cache_dir: PathBuf,
    pool: SqlitePool,
}

impl CacheHandler {
    pub async fn new() -> anyhow::Result<Self> {
        let cache_dir = if cfg!(debug_assertions) {
            std::env::current_dir()?.join("debug-cache")
        } else {
            let dirs = ProjectDirs::from("dev", "cordor", "ignoreit")
                .context("project directories creation")?;
            dirs.cache_dir().to_owned()
        };

        if !cache_dir.try_exists()? {
            std::fs::create_dir_all(&cache_dir)?;
        }

        let path = cache_dir.join(DB_PATH);
        if !path.try_exists()? {
            std::fs::File::create_new(&path)?;
        }

        let db_url = Url::from_file_path(&path).expect("valid file path url");
        let opts = SqliteConnectOptions::from_url(&db_url)?;
        let pool = SqlitePool::connect_with(opts).await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self { cache_dir, pool })
    }

    pub async fn update(&self, force: bool) -> anyhow::Result<bool> {
        let last_update = LastUpdate::get(self.cache_dir());

        match last_update {
            Ok(last_update) => {
                if force
                    || SystemTime::now()
                        .duration_since(last_update.time)
                        .expect("time to go forwards")
                        >= UPDATE_TIMEOUT
                {
                    self.update_inner(Some(last_update)).await?;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(error) => match error {
                update::Error::MissingLastUpdate => {
                    self.update_inner(None).await?;
                    Ok(true)
                }
                error => Err(error)?,
            },
        }
    }

    async fn update_inner(&self, last_update: Option<LastUpdate>) -> anyhow::Result<()> {
        let resp = reqwest::get(ALL_GITIGNORES).await?;
        let length = resp
            .content_length()
            .context("failed to get content length")?;
        let pb = indicatif::ProgressBar::new(length).with_message("Updating cache").with_style(ProgressStyle::with_template("{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?);
        let mut response_bytes = Vec::with_capacity(length as usize);
        let mut stream = resp.bytes_stream();
        while let Some(chunk_res) = stream.next().await {
            let chunk = chunk_res?;
            let chunk_len = chunk.len();
            response_bytes.append(&mut chunk.into());
            pb.inc(chunk_len as u64);
        }
        let raw_response = String::try_from(response_bytes)?;
        let mut response: Gitignores = serde_json::from_str(&raw_response)?;
        response.normalise();
        let update_data = LastUpdate::from_data(&response);
        update_data.save(self.cache_dir())?;

        let mut txn = self.pool.begin().await?;
        for row in response.values() {
            let query = if let Some(last_update) = last_update {
                if last_update.hash != update_data.hash {
                    Some(sqlx::query!(
                        r#"
						UPDATE gitignores
						SET contents = ?2, file_name = ?3, name = ?4, is_patch = ?5
						WHERE key = ?1
						"#,
                        row.key,
                        row.contents,
                        row.file_name,
                        row.name,
                        i64::from(row.is_patch),
                    ))
                } else {
                    None
                }
            } else {
                Some(sqlx::query!(
                    r#"
					INSERT INTO gitignores ( key, contents, file_name, name, is_patch )
					VALUES ( ?1, ?2, ?3, ?4, ?5 )
					"#,
                    row.key,
                    row.contents,
                    row.file_name,
                    row.name,
                    i64::from(row.is_patch),
                ))
            };

            if let Some(query) = query {
                let _: SqliteQueryResult = query.execute(&mut *txn).await?;
            }
        }

        txn.commit().await?;

        Ok(())
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// Purge the current cache
    pub fn purge(&self) -> anyhow::Result<()> {
        fs::remove_dir_all(self.cache_dir()).context("Failed to purge cache")?;

        Ok(())
    }

    /// Get a given template by name and return it's byte representation
    pub async fn get_template(&self, name: impl AsRef<str>) -> anyhow::Result<Gitignore> {
        let mut conn = self.pool.acquire().await?;
        let name = name.as_ref();
        let query: Gitignore = sqlx::query_as!(
            Gitignore,
            r#"
			SELECT * FROM gitignores
			WHERE key = ?;
			"#,
            name
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(query)
    }

    // todo: maybe stream this into tui??
    /// List all of the templates in the cache
    pub async fn list_templates(&self) -> anyhow::Result<Vec<Gitignore>> {
        let mut conn = self.pool.acquire().await?;
        let query: Vec<Gitignore> = sqlx::query_as!(
            Gitignore,
            r#"
			SELECT * FROM gitignores;
        	"#
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(query)
    }

    pub async fn search_templates(
        &self,
        search_query: impl AsRef<str>,
    ) -> anyhow::Result<Vec<String>> {
        let mut conn = self.pool.acquire().await?;
        let search_query = format!("%{}%", search_query.as_ref());
        let query: Vec<Gitignore> = sqlx::query_as!(
            Gitignore,
            r#"
			SELECT * FROM gitignores
			WHERE name LIKE ?;
			"#,
            search_query,
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(query.iter().map(|row| row.name.clone()).collect())
    }
}
