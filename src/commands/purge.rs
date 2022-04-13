use crate::cache::CACHE_DIR;
use anyhow::Context;
use std::fs;

pub fn purge() -> anyhow::Result<()> {
    let cache_dir = CACHE_DIR.to_owned().context("Failed to parse cache dir")?;

    fs::remove_dir_all(cache_dir).with_context(|| "Failed to purge cache")?;

    print!("Purged Cache!");

    Ok(())
}
