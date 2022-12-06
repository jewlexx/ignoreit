//! Handle gitignore cache

use std::{
    fs::{self, read_to_string, DirEntry},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Context;
use directories::BaseDirs;
use lazy_static::lazy_static;
use parking_lot::{const_mutex, Mutex};
use rayon::prelude::*;

lazy_static! {
    /// The directory containing the cache
    pub static ref CACHE_DIR: PathBuf = BaseDirs::new()
        .expect("failed to find cache dir")
        .cache_dir()
        .join("gitignore");

    /// If the cache is enabled or not
    pub static ref CACHE_ENABLED: bool = {
        let mut dir = CACHE_DIR.clone();
        dir.pop();
        dir.exists()
    };
}

/// Purge the current cache
pub fn purge() -> anyhow::Result<()> {
    let cache_dir = CACHE_DIR.clone();

    if cache_dir.exists() {
        fs::remove_dir_all(cache_dir).context("Failed to purge cache")?;
    } else {
        println!("No cache to purge.");
    }

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
        let mut has_recursed = HAS_RECURSED.lock();
        if *has_recursed > 2 {
            anyhow::bail!("Recursed too much during cache initialization");
        }

        *has_recursed += 1;
    }

    let fetch_path = CACHE_DIR.join(".timestamp");
    let cache_dir = CACHE_DIR.clone();

    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)?;
        fs::File::create(&fetch_path)?.write_all(crate::TIMESTAMP.to_string().as_bytes())?;
        return clone_templates();
    }

    if !fetch_path.exists() {
        fs::remove_dir_all(&cache_dir)?;
        return init_cache();
    }

    let timestamp_string = read_to_string(fetch_path)?;
    let timestamp = timestamp_string.parse::<u128>()?;
    let now = *crate::TIMESTAMP;

    let since = now - timestamp;

    if since >= TO_UPDATE {
        fs::remove_dir_all(cache_dir)?;
        clone_templates()?;
    }

    Ok(())
}

/// Get a given template by name and return it's byte representation
pub fn get_template(name: &TemplatePath) -> anyhow::Result<Vec<u8>> {
    let path = CACHE_DIR.join(&name.capped);

    if !path.exists() {
        Err(anyhow::anyhow!("Template not found"))
    } else {
        let mut file = fs::File::open(path).with_context(|| "Failed to open template file")?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        Ok(bytes)
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

/// List all of the templates in the cache
pub fn get_template_paths() -> anyhow::Result<Vec<TemplatePath>> {
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

fn clone_templates() -> anyhow::Result<()> {
    use std::io::Cursor;

    use indicatif::{ProgressBar, ProgressStyle};
    use zip::ZipArchive;

    const DOWNLOAD_URL: &str = "https://github.com/toptal/gitignore/archive/refs/heads/master.zip";
    let templates = crate::templates::github::GithubApi::new()?;
    let cache_dir = CACHE_DIR.clone();

    let downloaded = reqwest::blocking::get(DOWNLOAD_URL)?.bytes()?;

    let mut zip = ZipArchive::new(Cursor::new(downloaded))?;

    let file_names = zip
        .file_names()
        .par_bridge()
        .filter_map(|file_name| {
            if file_name.ends_with(".gitignore") {
                Some(file_name.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let style = ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({pos}/{len}, ETA {eta})",
    )?;

    let bar = ProgressBar::new(file_names.len() as u64);

    for zip_file_name in file_names {
        let file_name = zip_file_name.split('/').last().unwrap();
        let file_path = cache_dir.join(file_name);

        let file = zip.by_name(&zip_file_name).unwrap();

        let bytes = file.bytes();

        let file_bytes = bytes.collect::<Result<Vec<_>, _>>().unwrap();

        fs::write(file_path, file_bytes).unwrap();

        bar.inc(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    /// One day in milliseconds
    const DAY: u128 = 86400000;

    #[test]
    fn test_to_update() {
        assert_eq!(DAY, super::TO_UPDATE);
    }
}
