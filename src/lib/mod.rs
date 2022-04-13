use anyhow::Context;
use std::{collections::HashMap, fs};

use cache::CACHE_DIR;

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
        Err(anyhow::anyhow!("Failed to parse cache dir"))
    }
}
