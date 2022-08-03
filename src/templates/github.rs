use std::collections::HashMap;

pub type Gitignores = Vec<String>;

pub struct GithubApi {
    response: Gitignores,
}

impl GithubApi {
    // TODO: Implement a better error type
    pub fn new() -> anyhow::Result<Self> {
        const API_URL: &str = "https://api.github.com/gitignore/templates";

        let response: Gitignores = reqwest::blocking::get(API_URL)?.json()?;

        Ok(Self { response })
    }
}
