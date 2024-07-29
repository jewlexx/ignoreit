use std::{fmt::Display, path::PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum Category {
    #[allow(dead_code)]
    Global,
    #[allow(dead_code)]
    Community,
    Root,
}

#[derive(Debug, Clone)]
pub struct Template {
    name: String,
    path: PathBuf,
    #[allow(dead_code)]
    category: Category,
}

impl Template {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .with_extension("")
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        Template {
            name,
            path,
            category: Category::Root,
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
        write!(f, "{}", self.name)
    }
}
