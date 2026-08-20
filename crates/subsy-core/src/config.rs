use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub default_currency: Option<String>,
    #[serde(default)]
    pub default_reminder_days: i64,
    #[serde(default)]
    pub theme: Theme,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    #[default]
    Default,
    Dark,
    Light,
    HighContrast,
    Monochrome,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Default => "default",
            Theme::Dark => "dark",
            Theme::Light => "light",
            Theme::HighContrast => "high_contrast",
            Theme::Monochrome => "monochrome",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            "high_contrast" | "high-contrast" => Theme::HighContrast,
            "monochrome" => Theme::Monochrome,
            _ => Theme::Default,
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let s = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&s)?)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }
}
