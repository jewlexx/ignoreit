//! Global utils

use std::{path::PathBuf, time::SystemTime};

use directories::BaseDirs;
use lazy_static::lazy_static;

lazy_static! {
    /// The directory containing the cache
    pub static ref CACHE_DIR: PathBuf = BaseDirs::new()
        .expect("failed to find cache dir")
        .cache_dir()
        .join("gitignore");

    /// If the cache is enabled or not
    pub static ref CACHE_ENABLED: bool = {
        let mut dir = CACHE_DIR.clone();
        dir.pop();
        dir.exists()
    };

    /// The current time in milliseconds
    pub static ref TIMESTAMP: u128 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_millis();
}
