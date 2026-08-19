use directories::ProjectDirs;

pub fn data_dir() -> std::path::PathBuf {
    if let Some(dirs) = ProjectDirs::from("dev", "musichen", "subsy") {
        dirs.data_dir().to_path_buf()
    } else {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        std::path::PathBuf::from(home).join(".local/share/subsy")
    }
}

pub fn db_path() -> std::path::PathBuf {
    data_dir().join("subsy.db")
}

pub fn config_path() -> std::path::PathBuf {
    if let Some(dirs) = ProjectDirs::from("dev", "musichen", "subsy") {
        dirs.config_dir().to_path_buf().join("config.toml")
    } else {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        std::path::PathBuf::from(home).join(".config/subsy/config.toml")
    }
}
