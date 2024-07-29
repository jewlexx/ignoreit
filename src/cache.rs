use std::{ffi::OsStr, path::PathBuf};

use dialoguer::theme::ColorfulTheme;
use git2::{FetchOptions, RemoteCallbacks};

use crate::template::Template;

const GITIGNORE_REPO_URL: &str = "https://github.com/github/gitignore";

#[derive(Debug, Clone)]
pub struct Cache {
    templates: Vec<Template>,
}

impl Cache {
    pub fn path() -> Option<PathBuf> {
        crate::dirs::templates_dir()
    }

    pub fn open_repo(&self) -> anyhow::Result<CacheRepo> {
        let repo = git2::Repository::open(Self::path().unwrap())?;

        Ok(CacheRepo { repository: repo })
    }

    pub fn clone() -> anyhow::Result<Self> {
        let path = Self::path().unwrap();
        crate::clone::clone(GITIGNORE_REPO_URL, &path).unwrap();

        let files = Self::load_files()?;

        Ok(Self { templates: files })
    }

    pub fn load_files() -> anyhow::Result<Vec<Template>> {
        let mut files = Vec::new();

        for entry in walkdir::WalkDir::new(Self::path().unwrap()) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension() == Some(OsStr::new("gitignore")) {
                files.push(Template::new(path.to_path_buf()));
            }
        }

        Ok(files)
    }

    pub fn reload_files(&mut self) -> anyhow::Result<()> {
        self.templates = Self::load_files()?;

        Ok(())
    }

    pub fn open() -> anyhow::Result<Self> {
        // let repo = git2::Repository::open(Self::path().unwrap())?;

        let files = Self::load_files()?;

        Ok(Self { templates: files })
    }

    pub fn list_templates(&self) -> &[Template] {
        self.templates.as_ref()
    }

    pub fn pick_template(&self) -> anyhow::Result<Template> {
        let templates = self.list_templates();

        let chosen_index = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(templates)
            .with_prompt("Choose a gitignore template")
            .clear(true)
            .interact()?;

        Ok(templates[chosen_index].clone())
    }

    pub fn find_template(&self, name: &str) -> Option<Template> {
        self.templates
            .iter()
            .find(|t| t.name().to_lowercase() == name.to_lowercase())
            .cloned()
    }

    pub fn purge(&self) -> anyhow::Result<()> {
        let path = Self::path().unwrap();

        std::fs::remove_dir_all(&path)?;

        Ok(())
    }
}

pub struct CacheRepo {
    repository: git2::Repository,
}

impl CacheRepo {
    pub fn outdated(&self) -> anyhow::Result<bool> {
        let repo = &self.repository;

        // Setup callbacks for fetching
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| {
            // Modify this to provide your credentials if necessary
            git2::Cred::default()
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        // Fetch updates from the remote
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(
            &["refs/heads/*:refs/heads/*"],
            Some(&mut fetch_options),
            None,
        )?;

        let head = repo.head()?;
        let current_branch = head
            .shorthand()
            .ok_or(git2::Error::from_str("Invalid branch name"))?;

        // Get the local and remote references
        let local_ref = repo.head()?;
        let local_commit = local_ref.peel_to_commit()?;
        let remote_ref = repo.find_reference(&format!("refs/remotes/origin/{current_branch}"))?;
        let remote_commit = remote_ref.peel_to_commit()?;

        // Compare commits
        Ok(local_commit.id() != remote_commit.id())
    }
}
