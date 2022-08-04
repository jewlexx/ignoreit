//! Handle gitignore cache

use std::{
    fs::{self, read_to_string, DirEntry},
    io::{Read, Write},
};

use anyhow::Context;
use parking_lot::{const_mutex, Mutex};

use crate::utils::CACHE_DIR;

/// Purge the current cache
pub fn purge() -> anyhow::Result<()> {
    let cache_dir = CACHE_DIR.clone();

    fs::remove_dir_all(cache_dir).context("Failed to purge cache")?;

    Ok(())
}

/// One Day in seconds
///
/// 24 hours -> in minutes -> in seconds -> in milliseconds
const TO_UPDATE: u128 = 24 * 60 * 60 * 1000;

static HAS_RECURSED: Mutex<usize> = const_mutex(0);

/// Initialize cache
pub fn init_cache() -> anyhow::Result<()> {
    {
        if *HAS_RECURSED.lock() > 2 {
            anyhow::bail!("Recursed too much during cache initialization");
        }

        *HAS_RECURSED.lock() += 1;
    }

    let fetch_path = CACHE_DIR.join(".timestamp");
    let cache_dir = CACHE_DIR.clone();

    if !CACHE_DIR.exists() {
        fs::create_dir_all(&cache_dir)?;
        fs::File::create(&fetch_path)?.write_all(crate::utils::TIMESTAMP.to_string().as_bytes())?;
        return clone_templates();
    }

    if !fetch_path.exists() {
        fs::remove_dir_all(&cache_dir)?;
        return init_cache();
    }

    let timestamp_string = read_to_string(fetch_path)?;
    let timestamp = timestamp_string.parse::<u128>()?;
    let now = *crate::utils::TIMESTAMP;

    let since = now - timestamp;

    if since >= TO_UPDATE {
        fs::remove_dir_all(cache_dir)?;
        clone_templates()?;
    }

    Ok(())
}

/// Get a given template by name and return it's byte representation
pub fn get_template(name: &str) -> anyhow::Result<Vec<u8>> {
    let filename = name.to_owned() + ".gitignore";

    let path = CACHE_DIR.join(filename);

    if !path.exists() {
        return Err(anyhow::anyhow!("Template not found"));
    } else {
        let mut file = fs::File::open(path).with_context(|| "Failed to open template file")?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        Ok(bytes)
    }
}

/// List all of the templates in the cache
pub fn get_template_paths() -> anyhow::Result<Vec<String>> {
    let dir: Vec<DirEntry> = fs::read_dir::<&std::path::Path>(CACHE_DIR.as_ref())
        .context("Failed to read cache directory")?
        .collect::<Result<_, _>>()?;

    let ignores = dir
        .iter()
        .filter(|entry| {
            entry.file_type().unwrap().is_file()
                && entry.file_name().to_str().unwrap() != ".timestamp"
        })
        .map(|entry| {
            let file_name = entry.file_name();

            file_name
                .to_str()
                .expect("invalid utf-8 file name")
                .to_string()
        })
        .collect();

    Ok(ignores)
}

fn clone_templates() -> anyhow::Result<()> {
    let templates = crate::templates::github::GithubApi::new()?;
    let cache_dir = CACHE_DIR.clone();

    for gitignore in templates.response {
        let path = gitignore.path(&cache_dir);

        if !path.exists() {
            fs::create_dir_all(path.parent().context("Path was root for some reason")?)
                .context("Failed to create dir")?;
            let mut file = fs::File::create(path).context("Failed to create file")?;

            file.write_all(gitignore.bytes())?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_update() {
        assert_eq!(86400000, super::TO_UPDATE);
    }
}
