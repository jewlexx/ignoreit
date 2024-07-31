use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::cache::Cache;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Category {
    Subfolder(VecDeque<String>),
    Root,
}

impl Category {
    pub fn is_root(&self) -> bool {
        matches!(self, Category::Root)
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Subfolder(components) => {
                for component in components {
                    write!(f, "{}/", component)?;
                }
            }
            Category::Root => {}
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    name: String,
    path: PathBuf,
    category: Category,
}

impl PartialOrd for Template {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for Template {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl Template {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .with_extension("")
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let category = if let Some(category) = path
            .strip_prefix(Cache::path().unwrap())
            .ok()
            .and_then(|p| p.parent())
            .map(|category| {
                category
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().to_string())
                    .collect::<VecDeque<String>>()
            })
            .and_then(|category| {
                if category.is_empty() {
                    None
                } else {
                    Some(category)
                }
            }) {
            Category::Subfolder(category)
        } else {
            Category::Root
        };

        Template {
            name,
            path,
            category,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn category(&self) -> &Category {
        &self.category
    }

    pub fn category_mut(&mut self) -> &mut Category {
        &mut self.category
    }

    pub fn set_category(&mut self, category: Category) {
        self.category = category;
    }

    /// Returns the path relative to the cache folder
    pub fn relative_path(&self) -> Option<PathBuf> {
        self.given_relative_path(Cache::path().unwrap())
    }

    pub fn given_relative_path(&self, path: impl AsRef<Path>) -> Option<PathBuf> {
        self.path.strip_prefix(path).ok().map(ToOwned::to_owned)
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.category, f)?;
        Display::fmt(&self.name, f)?;

        Ok(())
    }
}
