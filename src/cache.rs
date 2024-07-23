use std::path::PathBuf;

use git2::{FetchOptions, RemoteCallbacks};

use crate::progress::CounterProgress;

const GITIGNORE_REPO_URL: &str = "https://github.com/github/gitignore";

pub struct Cache {
    cache_path: PathBuf,
    templates: Vec<PathBuf>,
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

        Ok(Self {
            cache_path: path,
            templates: files,
        })
    }

    pub fn load_files() -> anyhow::Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in walkdir::WalkDir::new(Self::path().unwrap()) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().unwrap() == "gitignore" {
                files.push(path.to_path_buf());
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

        Ok(Self {
            cache_path: Self::path().unwrap(),
            templates: files,
        })
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
