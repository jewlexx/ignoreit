use directories::BaseDirs;

lazy_static! {
    pub static ref USER_DIRS: Option<BaseDirs> = BaseDirs::new();
    pub static ref CACHE_ENABLED: bool = {
        if let Some(user_dirs) = USER_DIRS.to_owned() {
            user_dirs.cache_dir().exists()
        } else {
            false
        }
    };
}
