use anyhow::Context;
use directories::BaseDirs;
use git2::Repository;
use std::path::PathBuf;

lazy_static! {
    pub static ref DIRS: Option<BaseDirs> = BaseDirs::new();
    pub static ref CACHE_ENABLED: bool = {
        if let Some(dirs) = DIRS.to_owned() {
            dirs.cache_dir().exists()
        } else {
            false
        }
    };
    pub static ref CACHE_DIR: Option<PathBuf> = DIRS
        .to_owned()
        .map(|dirs| dirs.cache_dir().join("gitignore"));
}

const IGNORE_URL: &str = "https://github.com/github/gitignore.git";

pub fn init_cache() -> anyhow::Result<PathBuf> {
    if let Some(cache_dir) = CACHE_DIR.to_owned() {
        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir)
                .with_context(|| "Failed to create cache directory")?;
        }

        Repository::clone(IGNORE_URL, &cache_dir)
            .with_context(|| "Failed to clone gitignore repository")?;

        let fetch_head = cache_dir.join(".git/FETCH_HEAD");
        let meta = fetch_head.metadata()?;
        let last_modified = meta.modified()?;

        Ok(cache_dir)
    } else {
        Err(anyhow::anyhow!("Cache directory not found"))
    }
}
