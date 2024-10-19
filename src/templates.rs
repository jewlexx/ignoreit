use std::{collections::HashMap, path::Path, sync::LazyLock};

use include_dir::{include_dir, Dir};

pub static RAW_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/templates");

pub static TEMPLATES: LazyLock<Vec<Template>> =
    LazyLock::new(|| Template::from_dir(&RAW_TEMPLATES));

pub type File = include_dir::File<'static>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    file: File,
    patch: Option<File>,
}

impl Template {
    pub fn from_dir(dir: &'static Dir) -> Vec<Self> {
        let mut patches = HashMap::<String, Option<File>>::new();

        for file in dir.files() {
            let path = file.path();
            let ext = path.extension();
            let file_name = path
                .file_name()
                .expect("valid utf8 file name")
                .to_string_lossy()
                .to_string();

            if ext == Some(std::ffi::OsStr::new("patch")) {
                patches.entry(file_name).or_default().insert(file.clone());
            } else if ext == Some(std::ffi::OsStr::new("gitignore")) {
                patches.entry(file_name).or_default();
            }
        }

        let templates = patches.into_iter().map(|(name, patch)| {
            let file_path = Path::new(&name).with_extension("gitignore");
            let file = dir.get_file(file_path);

            Template {
                file: file.unwrap().clone(),
                patch,
            }
        });

        templates.collect()
    }
}
