use dirs::data_local_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("AppData path non trovato")]
    NoAppData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub sites: Vec<String>,
    pub pin_hash: Option<String>,
    pub block_doh: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            sites: vec![
                "youtube.com".into(),
                "www.youtube.com".into(),
                "m.youtube.com".into(),
                "youtu.be".into(),
            ],
            pin_hash: None,
            block_doh: true,
        }
    }
}

pub fn config_path() -> Result<PathBuf, ConfigError> {
    let base = data_local_dir().ok_or(ConfigError::NoAppData)?;
    Ok(base.join("SiteBlocker").join("config.json"))
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let path = config_path()?;
    if !path.exists() {
        let cfg = AppConfig::default();
        save_config(&cfg)?;
        return Ok(cfg);
    }
    let content = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&content)?)
}

pub fn save_config(config: &AppConfig) -> Result<(), ConfigError> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(config)?;
    fs::write(&path, json)?;
    Ok(())
}
