pub fn project_dirs() -> Option<directories::ProjectDirs> {
    directories::ProjectDirs::from("com", "jewlexx", "ignoreit")
}

pub fn cache_dir() -> Option<std::path::PathBuf> {
    let dirs = project_dirs()?;

    let path = dirs.cache_dir();

    if !path.exists() {
        std::fs::create_dir_all(path).ok()?;
    }

    Some(path.to_owned())
}

pub fn config_dir() -> Option<std::path::PathBuf> {
    let dirs = project_dirs()?;

    let path = dirs.config_dir();

    if !path.exists() {
        std::fs::create_dir_all(path).ok()?;
    }

    Some(path.to_owned())
}
