//! Handle gitignore cache

// TODO: Refactor into cache struct

use std::{
    fs::{self, DirEntry},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;

use directories::BaseDirs;

pub struct CacheHandler {
    cache_dir: PathBuf,
}

impl CacheHandler {
    pub fn new() -> Option<Self> {
        let dirs = BaseDirs::new()?;
        let cache_dir = dirs.cache_dir();

        cache_dir.exists().then(|| {
            let project_cache_dir = cache_dir.join("gitignore");

            CacheHandler {
                cache_dir: project_cache_dir,
            }
        })
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    pub fn fetch_path(&self) -> PathBuf {
        self.cache_dir().join(".hash")
    }

    /// Purge the current cache
    pub fn purge(&self) -> anyhow::Result<()> {
        fs::remove_dir_all(self.cache_dir()).context("Failed to purge cache")?;

        Ok(())
    }

    /// Initialize cache
    pub fn init_cache(&self) -> anyhow::Result<()> {
        unsafe fn check_recursion() -> anyhow::Result<()> {
            static mut HAS_RECURSED: usize = 0;

            if HAS_RECURSED > 2 {
                anyhow::bail!("Recursed too much during cache initialization");
            }

            HAS_RECURSED += 1;

            Ok(())
        }

        unsafe { check_recursion() }?;

        let cache_dir = self.cache_dir();
        let fetch_path = self.fetch_path();

        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
            fs::File::create(&fetch_path)?
                .write_all(crate::STARTUP_TIMESTAMP.as_millis().to_string().as_bytes())?;
            return self.clone_templates();
        }

        if !fetch_path.exists() {
            fs::remove_dir_all(&cache_dir)?;
            return self.init_cache();
        }

        let hash = fs::read_to_string(fetch_path)?;

        let client = reqwest::blocking::Client::builder()
            .user_agent("ignoreit")
            .build()?;

        let response = client
            .get("https://api.github.com/repos/github/gitignore/commits/main")
            .send()?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch gitignore template. Status code: {}, body: {}",
                response.status(),
                response.text()?
            ));
        }

        let response: serde_json::Value = serde_json::from_str(&response.text()?)?;
        let hash_value = response.get("sha").and_then(|sha| sha.as_str());

        if hash_value != Some(&hash) {
            fs::remove_dir_all(cache_dir)?;
            self.clone_templates()?;
        }

        Ok(())
    }

    /// Get a given template by name and return it's byte representation
    pub fn get_template(&self, name: &TemplatePath) -> anyhow::Result<Vec<u8>> {
        let path = self.cache_dir().join(&name.capped);

        if !path.exists() {
            Err(anyhow::anyhow!("Template not found"))
        } else {
            let mut file = fs::File::open(path).with_context(|| "Failed to open template file")?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;

            Ok(bytes)
        }
    }

    /// List all of the templates in the cache
    pub fn get_template_paths(&self) -> anyhow::Result<Vec<TemplatePath>> {
        let dir: Vec<DirEntry> = fs::read_dir(self.cache_dir())
            .context("Failed to read cache directory")?
            .collect::<Result<_, _>>()?;

        let ignores = dir
            .iter()
            .filter(|entry| {
                entry.file_type().unwrap().is_file()
                    && entry.file_name().to_str().unwrap() != ".hash"
            })
            .map(|entry| {
                let file_name = entry.file_name();
                let capped = file_name
                    .to_str()
                    .expect("invalid utf-8 file name")
                    .to_string();

                let lower = capped.to_lowercase();

                TemplatePath { lower, capped }
            })
            .collect();

        Ok(ignores)
    }

    fn clone_templates(&self) -> anyhow::Result<()> {
        let templates = crate::templates::github::GithubApi::new()?;
        let cache_dir = self.cache_dir();

        let client = reqwest::blocking::Client::builder()
            .user_agent("ignoreit")
            .build()?;

        let hash: serde_json::Value = client
            .get("https://api.github.com/repos/github/gitignore/commits/main")
            .send()?
            .json()?;

        for gitignore in templates.response {
            // This is allowed because removing the borrow will create an error
            let path = gitignore.path(&cache_dir);

            if !path.exists() {
                fs::create_dir_all(path.parent().context("Path was root for some reason")?)
                    .context("Failed to create dir")?;
                let mut file = fs::File::create(path).context("Failed to create file")?;

                file.write_all(gitignore.bytes())?;
            }
        }

        fs::File::create(cache_dir.join(".hash"))?
            .write_all(hash.get("sha").unwrap().as_str().unwrap().as_bytes())?;

        Ok(())
    }
}

#[derive(PartialEq, Eq)]
/// Structural representation of a template path
pub struct TemplatePath {
    /// Lowercase name
    pub lower: String,
    /// Cased name
    pub capped: String,
}

impl std::fmt::Display for TemplatePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.capped)
    }
}
