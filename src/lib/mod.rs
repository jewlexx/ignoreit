use anyhow::Context;
use reqwest::{blocking::Response, header::USER_AGENT};
use serde_json::Value;
use std::{collections::HashMap, fs};

use crate::cache::CACHE_DIR;

#[macro_use]
pub mod macros;

pub fn get_url(str: &str) -> anyhow::Result<Response> {
    let client = create_client!();

    let res = client
        .get(str)
        .header(USER_AGENT, "Gitignore Generator")
        .send()
        .with_context(|| "Failed to send request")?;

    if !res.status().is_success() {
        panic!("Failed to get response: {}", res.status())
    }

    Ok(res)
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
            .map(|e| {
                let entry = e.unwrap();
                let file_name = entry.file_name();
                let name = file_name
                    .to_str()
                    .unwrap()
                    .split('.')
                    .next()
                    .unwrap()
                    .to_owned();

                (name.to_lowercase(), name)
            })
            .collect::<Vec<(String, String)>>();

        let mut ignores = HashMap::<String, String>::new();

        for (lower, path) in ignores_tuple {
            ignores.insert(lower, path);
        }

        return Ok(ignores);
    }

    use spinners::{Spinner, Spinners};

    let sp = Spinner::new(Spinners::Dots12, "Fetching templates...".into());
    let body: Value = get_url("https://api.github.com/repos/github/gitignore/git/trees/main")?
        .json()
        .with_context(|| "Failed to read JSON from response")?;

    let tree = body["tree"]
        .as_array()
        .with_context(|| "Failed to parse tree")?;

    let tree = tree.iter().filter(|el| {
        let name = el["path"]
            .as_str()
            .with_context(|| "Failed to parse path")
            .unwrap();

        name.ends_with(".gitignore")
    });

    let mut hashmap: HashMap<String, String> = HashMap::new();

    for item in tree {
        let base_path = item["path"]
            .as_str()
            .with_context(|| "Failed to parse path")
            .unwrap();

        let path = base_path
            .split('.')
            .next()
            .with_context(|| "Failed to parse path")?;

        let lowercase = &path.to_lowercase();

        hashmap.insert(lowercase.to_string(), path.to_string());
    }

    sp.stop();

    Ok(hashmap)
}
