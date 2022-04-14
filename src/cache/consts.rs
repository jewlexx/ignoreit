use directories::BaseDirs;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    pub static ref CACHE_DIR: Option<PathBuf> =
        BaseDirs::new().map(|dirs| dirs.cache_dir().to_owned().join("gitignore"));
    pub static ref CACHE_ENABLED: bool = {
        // TODO: Fix this
        true
        // if let Some(dir) = CACHE_DIR.to_owned() {
        //     dir.exists()
        // } else {
        //     false
        // }
    };
}
