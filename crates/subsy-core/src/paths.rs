use std::path::PathBuf;

fn home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

pub fn data_dir() -> PathBuf {
    if let Ok(d) = std::env::var("SUBSY_DATA_DIR") {
        if !d.is_empty() {
            return PathBuf::from(d);
        }
    }
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        if !xdg.is_empty() {
            return PathBuf::from(xdg).join("subsy");
        }
    }
    home().join(".local/share/subsy")
}

pub fn config_dir() -> PathBuf {
    if let Ok(d) = std::env::var("SUBSY_CONFIG_DIR") {
        if !d.is_empty() {
            return PathBuf::from(d);
        }
    }
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return PathBuf::from(xdg).join("subsy");
        }
    }
    home().join(".config/subsy")
}

pub fn db_path() -> PathBuf {
    data_dir().join("subsy.db")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}
