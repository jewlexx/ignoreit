use directories::BaseDirs;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    static ref DIRS: Option<BaseDirs> = BaseDirs::new();
    pub static ref CACHE_ENABLED: bool = {
        if let Some(dirs) = DIRS.to_owned() {
            dirs.cache_dir().exists()
        } else {
            false
        }
    };
    pub static ref CACHE_DIR: Option<PathBuf> = DIRS
        .to_owned()
        .map(|dirs| dirs.cache_dir().join("gitignore"));
}
