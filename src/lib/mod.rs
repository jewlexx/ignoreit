use anyhow::Context as _;
use reqwest::{blocking::Response, header::USER_AGENT};
use serde_json::Value;
use std::collections::HashMap;

const TEMPLATES_URL: &str = "https://api.github.com/repos/github/gitignore/git/trees/main";

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
    let mut hashmap: HashMap<String, String> = HashMap::new();

    let body: Value = get_url(TEMPLATES_URL)?
        .json()
        .with_context(|| "Failed to read JSON from response")?;

    let tree = body["tree"]
        .as_array()
        .with_context(|| "Failed to parse tree")?;

    let tree = tree.iter().filter(|el| {
        let name = el["path"].to_string();
        name.ends_with(".gitignore")
    });

    for item in tree {
        let base_path = item["path"].to_string();
        let path = base_path
            .split('.')
            .next()
            .with_context(|| "Failed to parse path")?;
        let lowercase = &path.to_lowercase();

        hashmap.insert(lowercase.to_string(), path.to_string());
    }

    Ok(hashmap)
}
