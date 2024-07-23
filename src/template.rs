use std::path::PathBuf;

pub enum Category {
    Global,
    Community,
    Root,
}

pub struct Template {
    name: String,
    path: PathBuf,
    category: Category,
}

impl Template {
    pub fn new(path: PathBuf) -> Self {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
