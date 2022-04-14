use anyhow::Context;
use git2::Repository;
use spinners::{Spinner, Spinners};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

mod purge;
use crate::lib::{CACHE_DIR, IS_ONLINE};
pub use purge::purge;

fn clone_cache(dir: &Path) -> anyhow::Result<Repository> {
    let sp = Spinner::new(Spinners::Dots12, "Initializing Cache...".into());

    purge()?;
    let repo = Repository::clone("https://github.com/github/gitignore.git", &dir)
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
        let last_modified = fetch_head.metadata()?.modified()?;
        let since_modified = SystemTime::now().duration_since(last_modified)?;

        // If the cache is older than a day, fetch the latest version, but not if the user is offline, for obvious reasons
        if since_modified.as_secs() > 60 * 60 * 24 && IS_ONLINE.to_owned() {
            clone_cache(&cache_dir)?;
        }

        Ok(cache_dir)
    } else {
        Err(anyhow::anyhow!("Cache directory not found"))
    }
}

pub fn get_templates() -> anyhow::Result<HashMap<String, String>> {
    if let Some(cache_dir) = CACHE_DIR.to_owned() {
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
    } else {
        crate::remote::get_templates()
    }
}
