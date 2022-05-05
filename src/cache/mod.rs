use std::{
    collections::HashMap,
    fs,
    io::Read,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use anyhow::Context;
use git2::Repository;
use spinners::{Spinner, Spinners};

mod purge;
use crate::lib::CACHE_DIR;
pub use purge::purge;

const TO_UPDATE: u64 = 360 * 24;

fn clone_repo(url: &str, cache_dir: &str) -> anyhow::Result<Repository> {
    let mut sp = Spinner::new(Spinners::Dots12, "Initializing Cache...".into());

    let r = Repository::clone(url, cache_dir)
        .with_context(|| "Failed to clone gitignore repository")?;

    sp.stop_with_message("Cache Initialized!".into());

    Ok(r)
}

pub fn init_cache() -> anyhow::Result<PathBuf> {
    if let Some(cache_dir) = CACHE_DIR.to_owned() {
        let url = "https://github.com/github/gitignore.git";
        let fetch_path = cache_dir.join(".git").join("FETCH_HEAD");

        if !fetch_path.exists() {
            fs::remove_dir_all(&cache_dir)?;
            clone_repo(url, cache_dir.to_str().unwrap())?;

            return Ok(cache_dir);
        }

        let fetch_meta = fetch_path.metadata()?;
        let last_modified = fetch_meta.modified()?;
        let now = SystemTime::now();

        let since = now
            .duration_since(last_modified)
            .unwrap_or(Duration::from_secs(TO_UPDATE))
            .as_secs();

        if since > TO_UPDATE {
            clone_repo(url, cache_dir.to_str().unwrap())?;
        }

        Ok(cache_dir)
    } else {
        Err(anyhow::anyhow!("User's cache directory not found"))
    }
}

pub fn get_templates() -> anyhow::Result<HashMap<String, String>> {
    let cache_dir = CACHE_DIR.to_owned().context("Cache directory not found")?;
    let dir = fs::read_dir(cache_dir).with_context(|| "Failed to read cache directory")?;

    let ignores_tuple = dir
        .filter(|e| {
            if let Ok(entry) = e {
                entry.file_type().unwrap().is_file()
                    && entry.file_name().to_str().unwrap().ends_with(".gitignore")
            } else {
                false
            }
        })
        .map(|e| -> anyhow::Result<(String, String)> {
            let entry = e?;
            let file_name = entry.file_name();
            let name = file_name
                .to_str()
                .context("Failed to parse file name")?
                .split('.')
                .next()
                .context("Failed to parse file name")?
                .to_owned();

            Ok((name.to_lowercase(), name))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut ignores = HashMap::<String, String>::new();

    for (lower, path) in ignores_tuple {
        ignores.insert(lower, path);
    }

    Ok(ignores)
}

pub fn get_template(name: &str) -> anyhow::Result<String> {
    let cache_dir = CACHE_DIR.to_owned().context("Cache directory not found")?;
    let filename = name.to_owned() + ".gitignore";

    let path = cache_dir.join(filename);

    if !path.exists() {
        return Err(anyhow::anyhow!("Template not found"));
    } else {
        let file = fs::File::open(path).with_context(|| "Failed to open template file")?;
        let mut str = String::new();
        file.read_to_string(&mut str);

        Ok(str)
    }
}
