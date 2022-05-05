use std::path::PathBuf;

use directories::BaseDirs;
use lazy_static::lazy_static;

use crate::{sleep_for, sys::is_online};

lazy_static! {
    pub static ref CACHE_DIR: Option<PathBuf> =
        BaseDirs::new().map(|dirs| dirs.cache_dir().to_owned().join("gitignore"));
    pub static ref CACHE_ENABLED: bool = {
        if let Some(mut dir) = CACHE_DIR.to_owned() {
            dir.pop();
            dir.exists()
        } else {
            false
        }
    };
    pub static ref IS_ONLINE: bool = {
        if !is_online() {
            use colored::Colorize;
            println!("{}","warning: you are offline. you will only be able to use cached templates which may be out of date".yellow());
            sleep_for!(3000);

            true
        } else {
            false
        }
    };
}
