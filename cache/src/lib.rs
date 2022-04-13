use anyhow::Context;
use directories::BaseDirs;
use git2::Repository;
use spinners::{Spinner, Spinners};
use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

mod purge;
pub use purge::purge;

#[macro_use]
extern crate lazy_static;

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

fn clone_cache(dir: &Path) -> anyhow::Result<Repository> {
    let sp = Spinner::new(Spinners::Dots12, "Initializing Cache...".into());

    fs::remove_dir_all(&dir).with_context(|| "Failed to remove cache directory")?;
    let repo = Repository::clone(IGNORE_URL, &dir)
        .with_context(|| "Failed to clone gitignore repository")?;

    sp.stop_with_newline();

    Ok(repo)
}

pub fn init_cache() -> anyhow::Result<PathBuf> {
    if let Some(cache_dir) = CACHE_DIR.to_owned() {
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir).with_context(|| "Failed to create cache directory")?;
            clone_cache(&cache_dir)?;
        }

        let fetch_head = cache_dir.join(".git/FETCH_HEAD");
        let meta = fetch_head.metadata()?;
        let last_modified = meta.modified()?;
        let since_modified = SystemTime::now().duration_since(last_modified)?;

        // If the cache is older than a day, fetch the latest version
        if since_modified.as_secs() > 60 * 60 * 24 {
            clone_cache(&cache_dir)?;
        }

        Ok(cache_dir)
    } else {
        Err(anyhow::anyhow!("Cache directory not found"))
    }
}
