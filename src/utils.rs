use std::path::PathBuf;

use directories::BaseDirs;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CACHE_DIR: PathBuf = BaseDirs::new()
        .map(|dirs| dirs.cache_dir().to_owned().join("gitignore"))
        .expect("failed to find cache dir");
    pub static ref CACHE_ENABLED: bool = {
        let mut dir = CACHE_DIR.clone();
        dir.pop();
        dir.exists()
    };
}
