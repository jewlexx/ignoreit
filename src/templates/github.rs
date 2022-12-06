//! Interaction with Github

use std::path::{Path, PathBuf};

use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::parse_url;

/// Alias for Gitignore response
pub type Gitignores = Vec<String>;

/// Alias for Gitignore response
pub type GitignoreResponse = Vec<GitignoreFile>;

/// Struct representation of gitignore files
pub struct GitignoreFile {
    path: String,
    bytes: Vec<u8>,
}

impl GitignoreFile {
    /// Return a reference to the path
    pub fn path(&self, base: impl AsRef<Path>) -> PathBuf {
        base.as_ref().join(&self.path)
    }

    /// Return a reference to the bytes
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Struct representation of the Github API
pub struct GithubApi {
    /// The given response
    pub response: GitignoreResponse,
}

impl GithubApi {
    // TODO: Implement a better error type
    /// Get the response from Github
    pub fn new() -> anyhow::Result<Self> {
        const API_URL: &str = "https://www.toptal.com/developers/gitignore/api/list";

        let style = ProgressStyle::default_bar().template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )?;

        let text_response = reqwest::blocking::Client::builder()
            .user_agent("ignoreit")
            .build()?
            .get(API_URL)
            .send()?
            .text()?;

        let response: Gitignores = text_response.split(',').map(String::from).collect();

        let files = response
            .par_iter()
            .progress_with_style(style)
            .map(|template| {
                let download_url = parse_url!(template);

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
