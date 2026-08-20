use std::path::PathBuf;

/// App-internal directory: same folder as the running binary.
/// Override with `SUBSY_DATA_DIR` for tests or custom installs.
pub fn app_dir() -> PathBuf {
    if let Ok(d) = std::env::var("SUBSY_DATA_DIR") {
        if !d.is_empty() {
            return PathBuf::from(d);
        }
    }
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn db_path() -> PathBuf {
    app_dir().join("subsy.db")
}

pub fn config_path() -> PathBuf {
    app_dir().join("subsy-config.toml")
}
