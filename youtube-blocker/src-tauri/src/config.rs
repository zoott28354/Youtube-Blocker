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
                "www.youtu.be".into(),
                "m.youtu.be".into(),
            ],
            pin_hash: None,
            block_doh: true,
        }
    }
}

pub fn config_path() -> Result<PathBuf, ConfigError> {
    // %PROGRAMDATA% (C:\ProgramData) — condiviso tra tutti gli utenti Windows,
    // scrivibile solo con privilegi admin (che l'app già richiede).
    // Necessario per perMachine: il config PIN deve essere lo stesso
    // indipendentemente da quale account apre l'app.
    #[cfg(target_os = "windows")]
    {
        let programdata = std::env::var("PROGRAMDATA")
            .unwrap_or_else(|_| "C:\\ProgramData".into());
        Ok(PathBuf::from(programdata).join("YouTubeBlocker").join("config.json"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        let base = dirs::data_local_dir().ok_or(ConfigError::NoAppData)?;
        Ok(base.join("YouTubeBlocker").join("config.json"))
    }
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
