use anyhow::Context;
use directories::BaseDirs;
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

pub fn init_cache() -> anyhow::Result<()> {
    if !CACHE_ENABLED.to_owned() {
        return Ok(());
    }

    if let Some(cache_dir) = CACHE_DIR.to_owned() {
        if !cache_dir.exists() {
            std::fs::create_dir_all(&cache_dir).with_context(|| "Failed to create cache directory")
        } else {
            Ok(())
        }
    } else {
        Err(anyhow::anyhow!("Cache directory not found"))
    }
}
