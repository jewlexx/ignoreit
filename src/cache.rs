mod pick;

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use anyhow::Context;
use git2::{FetchOptions, RemoteCallbacks};

use crate::template::Template;

const GITIGNORE_REPO_URL: &str = "https://github.com/github/gitignore";

#[derive(Debug, Clone)]
pub struct Folder {
    name: String,
    files: Vec<Template>,
    folders: Vec<Folder>,
}

impl Folder {
    pub fn is_empty(&self) -> bool {
        self.files.is_empty() && self.folders.iter().all(|folder| folder.is_empty())
    }

    pub fn load_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let name = path.as_ref().file_name().context("missing file name")?;
        let mut files = vec![];
        let mut folders = vec![];

        let folder_read = std::fs::read_dir(&path)?;

        for entry in folder_read {
            let entry = entry?;
            let path = entry.path();
            if entry.path().is_dir() {
                folders.push(Folder::load_path(path)?);
            } else if path.extension() == Some(OsStr::new("gitignore")) {
                files.push(Template::new(path));
            }
        }

        let folder = Self {
            name: name.to_string_lossy().to_string(),
            files,
            folders,
        };

        Ok(folder.cleanup())
    }

    fn cleanup(self) -> Self {
        let mut cleaned_folders = vec![];

        for folder in self.folders {
            if !folder.is_empty() {
                cleaned_folders.push(folder.cleanup());
            }
        }

        Self {
            name: self.name,
            files: self.files,
            folders: cleaned_folders,
        }
    }

    pub fn list_templates(&self) -> &[Template] {
        &self.files
    }

    pub fn list_templates_recursively(&self) -> Vec<Template> {
        let mut templates = self.list_templates().to_vec();

        for folder in &self.folders {
            templates.extend(folder.list_templates_recursively());
        }

        templates
    }
}

#[derive(Debug, Clone)]
pub struct Cache {
    root: Folder,
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

        let root = Self::load_root()?;

        Ok(Self { root })
    }

    pub fn load_root() -> anyhow::Result<Folder> {
        let root_folder = Folder::load_path(Self::path().context("missing cache path")?)?;

        Ok(root_folder)
    }

    pub fn reload_files(&mut self) -> anyhow::Result<()> {
        self.root = Self::load_root()?;

        Ok(())
    }

    pub fn open() -> anyhow::Result<Self> {
        // let repo = git2::Repository::open(Self::path().unwrap())?;

        let root = Self::load_root()?;

        Ok(Self { root })
    }

    pub fn list_templates(&self) -> Vec<Template> {
        self.root.list_templates_recursively()
    }

    pub fn pick_template(&self) -> anyhow::Result<Option<Template>> {
        // let templates = self.list_templates();

        // let chosen_index = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
        //     .items(templates)
        //     .with_prompt("Choose a gitignore template")
        //     .clear(true)
        //     .interact()?;

        // Ok(templates[chosen_index].clone())

        pick::pick_template()
    }

    pub fn find_template(&self, name: &str) -> Option<Template> {
        self.root
            .list_templates_recursively()
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
