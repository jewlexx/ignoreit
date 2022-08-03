use std::{
    collections::HashMap,
    fs::{self, read_to_string, DirEntry},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;

use crate::utils::CACHE_DIR;

pub fn purge() -> anyhow::Result<()> {
    let cache_dir = CACHE_DIR.clone().context("Failed to parse cache dir")?;

    fs::remove_dir_all(cache_dir).context("Failed to purge cache")?;

    Ok(())
}

/// One Day in seconds
///
/// 24 hours -> in minutes -> in seconds -> in milliseconds
const TO_UPDATE: u128 = 24 * 60 * 60 * 1000;

fn clone_templates(cache_dir: &Path) -> anyhow::Result<()> {
    let templates = crate::templates::github::GithubApi::new()?;

    for gitignore in templates.response {
        let path = gitignore.path(cache_dir);

        if !path.exists() {
            fs::create_dir_all(path.parent().context("Path was root for some reason")?)
                .context("Failed to create dir")?;
            let mut file = fs::File::create(path).context("Failed to create file")?;

            file.write_all(gitignore.bytes())?;
        }
    }

    Ok(())
}

pub fn init_cache() -> anyhow::Result<PathBuf> {
    if let Some(cache_dir) = CACHE_DIR.clone() {
        let fetch_path = cache_dir.join(".timestamp");

        if !cache_dir.exists() {
            println!("No cache dir found, creating");
            fs::create_dir_all(cache_dir.clone())?;
            fs::File::create(&fetch_path)?.write_all(crate::TIMESTAMP.to_string().as_bytes())?;
            clone_templates(&cache_dir)?;
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
            fs::remove_dir_all(&cache_dir)?;
            clone_templates(&cache_dir)?;
        }

        Ok(cache_dir)
    } else {
        Err(anyhow::anyhow!("User's cache directory not found"))
    }
}

pub fn get_templates() -> anyhow::Result<HashMap<String, String>> {
    let cache_dir = CACHE_DIR.clone().context("Cache directory not found")?;
    let dir = fs::read_dir(cache_dir)
        .with_context(|| "Failed to read cache directory")?
        .collect::<Result<Vec<DirEntry>, _>>()?;

    let ignores_tuple = dir
        .filter(|entry| {
            entry.file_type().unwrap().is_file()
                && entry.file_name().to_str().unwrap().ends_with(".gitignore")
        })
        .map(|entry| -> anyhow::Result<(String, String)> {
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

pub fn get_template(name: &str) -> anyhow::Result<Vec<u8>> {
    let cache_dir = CACHE_DIR.clone().context("Cache directory not found")?;
    let filename = name.to_owned() + ".gitignore";

    let path = cache_dir.join(filename);

    if !path.exists() {
        return Err(anyhow::anyhow!("Template not found"));
    } else {
        let mut file = fs::File::open(path).with_context(|| "Failed to open template file")?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_update() {
        assert_eq!(86400000, super::TO_UPDATE);
    }
}
