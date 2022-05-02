use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Context;
use spinners::{Spinner, Spinners};

mod purge;
use crate::lib::CACHE_DIR;
pub use purge::purge;

pub fn init_cache() -> anyhow::Result<PathBuf> {
    if let Some(cache_dir) = CACHE_DIR.to_owned() {
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir).with_context(|| "Failed to create cache directory")?;
        }

        let sp = Spinner::new(Spinners::Dots12, "Initializing Cache...".into());

        let map = crate::remote::get_templates()?;
        let values: Vec<String> = map.values().cloned().collect();

        for value in values {
            let template_path = map
                .get(&value.to_lowercase())
                .with_context(|| "Template not found")?;

            crate::commands::pull::get_contents_remote(template_path)?;
        }

        sp.stop_with_message("Cache Initialized!".into());

        Ok(cache_dir)
    } else {
        Err(anyhow::anyhow!("User's cache directory not found"))
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
