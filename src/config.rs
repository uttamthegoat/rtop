use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub refresh_rate_ms: u64,
    pub theme: String,
    pub show_tree: bool,
    pub show_mouse: bool,
    pub compact_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            refresh_rate_ms: 1000,
            theme: "default".to_string(),
            show_tree: false,
            show_mouse: false,
            compact_mode: false,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::path();
        if let Some(path) = config_path {
            if path.exists() {
                let contents = std::fs::read_to_string(&path)?;
                let config: Config = toml::from_str(&contents)?;
                return Ok(config);
            }
        }
        Ok(Config::default())
    }

    pub fn save(&self) -> Result<()> {
        if let Some(path) = Self::path() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let contents = toml::to_string_pretty(self)?;
            std::fs::write(&path, contents)?;
        }
        Ok(())
    }

    fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("rtop").join("config.toml"))
    }
}
