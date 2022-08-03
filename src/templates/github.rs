use std::path::{Path, PathBuf};

use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub type Gitignores = Vec<String>;

pub type GitignoreResponse = Vec<GitignoreFile>;

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

        let style = ProgressStyle::default_bar().template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )?;

        let response: Gitignores = reqwest::blocking::Client::builder()
            .user_agent("ignoreit")
            .build()?
            .get(API_URL)
            .send()?
            .json()?;

        let files = response
            .par_iter()
            .progress_with_style(style)
            .map(|template| {
                let download_url = format!(
                    "https://raw.githubusercontent.com/github/gitignore/main/{}.gitignore",
                    template
                );

                let file = reqwest::blocking::get(download_url)?.bytes()?.to_vec();

                Ok::<_, reqwest::Error>(GitignoreFile {
                    path: template.clone(),
                    bytes: file,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { response: files })
    }
}
