use std::{cmp::Ordering, fmt::Display, path::PathBuf};

use crate::cache::Cache;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Category {
    Subfolder(Vec<String>),
    Root,
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
    #[allow(dead_code)]
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
        {
            Category::Subfolder(
                category
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().to_string())
                    .collect(),
            )
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
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.category, f)?;
        Display::fmt(&self.name, f)?;

        Ok(())
    }
}
