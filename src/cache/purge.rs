use std::fs;

use anyhow::Context;

use crate::lib::CACHE_DIR;

pub fn purge() -> anyhow::Result<()> {
    let cache_dir = CACHE_DIR.to_owned().context("Failed to parse cache dir")?;

    fs::remove_dir_all(cache_dir).with_context(|| "Failed to purge cache")?;

    Ok(())
}
