use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub type Gitignores = Vec<String>;

pub type GitignoreResponse = HashMap<String, Vec<u8>>;

pub struct GitignoreFile {
    path: String,
    bytes: Vec<u8>,
}

impl GitignoreFile {
    pub fn path(&self, base: impl AsRef<Path>) -> PathBuf {
        base.as_ref().join(&self.path)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

pub struct GithubApi {
    pub response: GitignoreResponse,
}

impl GithubApi {
    // TODO: Implement a better error type
    pub fn new() -> anyhow::Result<Self> {
        const API_URL: &str = "https://api.github.com/gitignore/templates";

        let response: Gitignores = reqwest::blocking::get(API_URL)?.json()?;

        let files: Vec<(String, Vec<u8>)> = response
            .par_iter()
            .progress_count(response.len() as u64)
            .map(|template| {
                let download_url = format!(
                    "https://raw.githubusercontent.com/github/gitignore/main/{}.gitignore",
                    template
                );

                let file = reqwest::blocking::get(download_url)?.bytes()?.to_vec();

                Ok::<_, reqwest::Error>((template.clone(), file))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut mapped = HashMap::new();

        for file in files {
            mapped.insert(file.0, file.1);
        }

        Ok(Self { response: mapped })
    }
}
