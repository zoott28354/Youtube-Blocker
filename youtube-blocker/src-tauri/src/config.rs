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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

fn expand_roots(roots: &[&str]) -> Vec<String> {
    let mut sites = Vec::new();
    for root in roots {
        sites.push((*root).to_string());
        sites.push(format!("www.{}", root));
        sites.push(format!("m.{}", root));
    }
    sites
}

pub fn predefined_lists() -> Vec<BlockList> {
    vec![
        BlockList {
            id: "builtin-youtube".into(),
            name: "YouTube & Video".into(),
            sites: expand_roots(&[
                "youtube.com",
                "youtu.be",
                "youtube-nocookie.com",
                "music.youtube.com",
                "youtubekids.com",
                "vimeo.com",
                "dailymotion.com",
                "twitch.tv",
                "kick.com",
            ]),
            active: true,
            builtin: true,
        },
        BlockList {
            id: "builtin-gaming".into(),
            name: "Browser Games".into(),
            sites: expand_roots(&[
                "poki.com",
                "crazygames.com",
                "friv.com",
                "y8.com",
                "kizi.com",
                "agame.com",
                "gamepix.com",
                "plays.org",
                "miniplay.com",
                "coolmathgames.com",
                "hoodamath.com",
            ]),
            active: false,
            builtin: true,
        },
        BlockList {
            id: "builtin-platforms".into(),
            name: "Gaming Platforms".into(),
            sites: expand_roots(&[
                "roblox.com",
                "fortnite.com",
                "epicgames.com",
                "minecraft.net",
                "steamcommunity.com",
            ]),
            active: false,
            builtin: true,
        },
        BlockList {
            id: "builtin-messaging".into(),
            name: "Chat & Messaging".into(),
            sites: expand_roots(&[
                "whatsapp.com",
                "web.whatsapp.com",
                "telegram.org",
                "web.telegram.org",
                "discord.com",
                "messenger.com",
            ]),
            active: false,
            builtin: true,
        },
        BlockList {
            id: "builtin-social".into(),
            name: "Social Media".into(),
            sites: expand_roots(&[
                "instagram.com",
                "facebook.com",
                "tiktok.com",
                "x.com",
                "twitter.com",
                "snapchat.com",
                "reddit.com",
                "pinterest.com",
            ]),
            active: false,
            builtin: true,
        },
        BlockList {
            id: "builtin-streaming".into(),
            name: "Streaming".into(),
            sites: expand_roots(&[
                "netflix.com",
                "disneyplus.com",
                "primevideo.com",
                "amazonvideo.com",
                "crunchyroll.com",
            ]),
            active: false,
            builtin: true,
        },
    ]
}

fn sync_predefined_lists(existing: &[BlockList]) -> Vec<BlockList> {
    let defaults = predefined_lists();
    let mut merged = Vec::new();

    for default in defaults {
        if let Some(current) = existing.iter().find(|list| list.id == default.id) {
            merged.push(BlockList {
                active: current.active,
                ..default
            });
        } else {
            merged.push(default);
        }
    }

    for list in existing {
        if !list.builtin {
            merged.push(list.clone());
        }
    }

    merged
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

    let synced_lists = sync_predefined_lists(&cfg.lists);
    if synced_lists != cfg.lists {
        cfg.lists = synced_lists;
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
