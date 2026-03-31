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
    #[cfg(not(target_os = "windows"))]
    #[error("AppData path non trovato")]
    NoAppData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockList {
    pub id: String,
    pub name: String,
    pub sites: Vec<String>,
    pub active: bool,
    pub builtin: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub lists: Vec<BlockList>,
    pub pin_hash: Option<String>,
    #[serde(default = "default_true")]
    pub block_doh: bool,
}

fn default_true() -> bool {
    true
}

pub fn predefined_lists() -> Vec<BlockList> {
    vec![
        BlockList {
            id: "builtin-youtube".into(),
            name: "YouTube".into(),
            sites: vec![
                "youtube.com".into(),
                "www.youtube.com".into(),
                "m.youtube.com".into(),
                "youtu.be".into(),
                "www.youtu.be".into(),
                "m.youtu.be".into(),
            ],
            active: true,
            builtin: true,
        },
        BlockList {
            id: "builtin-gaming".into(),
            name: "Giochi".into(),
            sites: vec![
                "roblox.com".into(),
                "www.roblox.com".into(),
                "m.roblox.com".into(),
                "fortnite.com".into(),
                "www.fortnite.com".into(),
                "epicgames.com".into(),
                "www.epicgames.com".into(),
                "minecraft.net".into(),
                "www.minecraft.net".into(),
            ],
            active: false,
            builtin: true,
        },
        BlockList {
            id: "builtin-social".into(),
            name: "Social Media".into(),
            sites: vec![
                "instagram.com".into(),
                "www.instagram.com".into(),
                "facebook.com".into(),
                "www.facebook.com".into(),
                "m.facebook.com".into(),
                "tiktok.com".into(),
                "www.tiktok.com".into(),
                "x.com".into(),
                "www.x.com".into(),
                "twitter.com".into(),
                "www.twitter.com".into(),
                "snapchat.com".into(),
                "www.snapchat.com".into(),
            ],
            active: false,
            builtin: true,
        },
        BlockList {
            id: "builtin-streaming".into(),
            name: "Streaming".into(),
            sites: vec![
                "netflix.com".into(),
                "www.netflix.com".into(),
                "twitch.tv".into(),
                "www.twitch.tv".into(),
                "disneyplus.com".into(),
                "www.disneyplus.com".into(),
                "primevideo.com".into(),
                "www.primevideo.com".into(),
                "crunchyroll.com".into(),
                "www.crunchyroll.com".into(),
            ],
            active: false,
            builtin: true,
        },
    ]
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            lists: predefined_lists(),
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
    let mut cfg: AppConfig = serde_json::from_str(&content)?;

    // Migration: vecchio formato (v1.0.x) aveva `sites: Vec<String>` invece di `lists`.
    // Se `lists` è vuota, convertiamo i vecchi siti in "Siti personalizzati" e
    // aggiungiamo le liste predefinite (inattive).
    if cfg.lists.is_empty() {
        let mut new_lists = predefined_lists();
        if let Ok(raw) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(sites_val) = raw.get("sites") {
                if let Ok(old_sites) = serde_json::from_value::<Vec<String>>(sites_val.clone()) {
                    if !old_sites.is_empty() {
                        new_lists.insert(
                            0,
                            BlockList {
                                id: "migrated-custom".into(),
                                name: "Siti personalizzati".into(),
                                sites: old_sites,
                                active: true,
                                builtin: false,
                            },
                        );
                    }
                }
            }
        }
        cfg.lists = new_lists;
        save_config(&cfg)?;
    }

    Ok(cfg)
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
