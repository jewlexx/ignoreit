use std::path::PathBuf;

use directories::BaseDirs;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CACHE_DIR: Option<PathBuf> =
        BaseDirs::new().map(|dirs| dirs.cache_dir().to_owned().join("gitignore"));
    pub static ref CACHE_ENABLED: bool = {
        if let Some(mut dir) = CACHE_DIR.clone() {
            dir.pop();
            dir.exists()
        } else {
            false
        }
    };
}
