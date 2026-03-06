#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod browsers;
mod config;
mod firewall;
mod hosts;

use auth::{hash_pin, verify_pin};
use browsers::{are_policies_active, disable_browser_doh, enable_browser_doh};
use config::{load_config, save_config, AppConfig};
use firewall::{add_firewall_rules, are_rules_active, remove_firewall_rules};
use hosts::{block_sites, is_blocked, unblock_sites};
use std::process::Command;
use std::sync::Mutex;
use tauri::State;

// --- Gestione privilegi admin ---

#[cfg(target_os = "windows")]
fn is_admin() -> bool {
    // "net session" fallisce senza privilegi admin
    Command::new("net")
        .args(["session"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(target_os = "windows")]
fn relaunch_as_admin() {
    use std::os::windows::process::CommandExt;
    let exe = std::env::current_exe().expect("impossibile trovare eseguibile");
    Command::new("powershell")
        .args([
            "-NoProfile",
            "-WindowStyle",
            "Hidden",
            "-Command",
            &format!("Start-Process '{}' -Verb RunAs", exe.to_string_lossy()),
        ])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .spawn()
        .expect("impossibile rilanciare con privilegi admin");
}

struct AppState(Mutex<AppConfig>);

#[derive(serde::Serialize)]
struct BlockStatus {
    hosts_blocked: bool,
    firewall_active: bool,
    browser_policy: bool,
}

// --- Comandi Tauri ---

#[tauri::command]
fn get_status(state: State<AppState>) -> Result<BlockStatus, String> {
    let cfg = state.0.lock().unwrap();
    let hosts_blocked = is_blocked(&cfg.sites).map_err(|e| e.to_string())?;
    let firewall_active = if cfg.block_doh { are_rules_active() } else { false };
    let browser_policy = if cfg.block_doh { are_policies_active() } else { false };
    Ok(BlockStatus {
        hosts_blocked,
        firewall_active,
        browser_policy,
    })
}

#[tauri::command]
fn get_sites(state: State<AppState>) -> Vec<String> {
    state.0.lock().unwrap().sites.clone()
}

/// Normalizza l'input e restituisce root + varianti www/m.
/// Es. "https://www.netflix.com/it" → ["netflix.com", "www.netflix.com", "m.netflix.com"]
fn expand_domain(input: &str) -> Vec<String> {
    let domain = input
        .trim()
        .to_lowercase()
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or("")
        .trim_start_matches("www.")
        .trim_start_matches("m.")
        .to_string();

    if domain.is_empty() || !domain.contains('.') {
        return vec![];
    }

    vec![
        domain.clone(),
        format!("www.{}", domain),
        format!("m.{}", domain),
    ]
}

#[tauri::command]
fn add_site(domain: String, state: State<AppState>) -> Result<(), String> {
    let mut cfg = state.0.lock().unwrap();
    let variants = expand_domain(&domain);
    if variants.is_empty() {
        return Err("Dominio non valido".into());
    }
    for variant in variants {
        if !cfg.sites.contains(&variant) {
            cfg.sites.push(variant);
        }
    }
    save_config(&cfg).map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_site(domain: String, state: State<AppState>) -> Result<(), String> {
    let mut cfg = state.0.lock().unwrap();
    let to_remove: std::collections::HashSet<String> = expand_domain(&domain).into_iter().collect();
    cfg.sites.retain(|s| !to_remove.contains(s));
    save_config(&cfg).map_err(|e| e.to_string())
}

#[tauri::command]
fn block_all(state: State<AppState>) -> Result<(), String> {
    let cfg = state.0.lock().unwrap();
    block_sites(&cfg.sites).map_err(|e| e.to_string())?;
    if cfg.block_doh {
        add_firewall_rules().map_err(|e| e.to_string())?;
        disable_browser_doh();
    }
    Ok(())
}

#[tauri::command]
fn unblock_all(pin: String, state: State<AppState>) -> Result<(), String> {
    let cfg = state.0.lock().unwrap();
    match &cfg.pin_hash {
        Some(hash) => verify_pin(&pin, hash).map_err(|e| e.to_string())?,
        None => return Err("Nessun PIN configurato".into()),
    }
    unblock_sites(&cfg.sites).map_err(|e| e.to_string())?;
    if cfg.block_doh {
        remove_firewall_rules().map_err(|e| e.to_string())?;
        enable_browser_doh();
    }
    Ok(())
}

#[tauri::command]
fn has_pin(state: State<AppState>) -> bool {
    state.0.lock().unwrap().pin_hash.is_some()
}

#[tauri::command]
fn set_pin(pin: String, state: State<AppState>) -> Result<(), String> {
    if pin.len() < 4 {
        return Err("Il PIN deve avere almeno 4 caratteri".into());
    }
    let mut cfg = state.0.lock().unwrap();
    let hash = hash_pin(&pin).map_err(|e| e.to_string())?;
    cfg.pin_hash = Some(hash);
    save_config(&cfg).map_err(|e| e.to_string())
}

#[tauri::command]
fn reset_pin(state: State<AppState>) -> Result<(), String> {
    let mut cfg = state.0.lock().unwrap();
    cfg.pin_hash = None;
    save_config(&cfg).map_err(|e| e.to_string())
}

#[tauri::command]
fn change_pin(old_pin: String, new_pin: String, state: State<AppState>) -> Result<(), String> {
    if new_pin.len() < 4 {
        return Err("Il PIN deve avere almeno 4 caratteri".into());
    }
    let mut cfg = state.0.lock().unwrap();
    match &cfg.pin_hash {
        Some(hash) => verify_pin(&old_pin, hash).map_err(|e| e.to_string())?,
        None => return Err("Nessun PIN impostato".into()),
    }
    let new_hash = hash_pin(&new_pin).map_err(|e| e.to_string())?;
    cfg.pin_hash = Some(new_hash);
    save_config(&cfg).map_err(|e| e.to_string())
}

fn main() {
    #[cfg(target_os = "windows")]
    if !is_admin() {
        relaunch_as_admin();
        return;
    }

    let config = load_config().unwrap_or_default();

    tauri::Builder::default()
        .manage(AppState(Mutex::new(config)))
        .invoke_handler(tauri::generate_handler![
            get_status,
            get_sites,
            add_site,
            remove_site,
            block_all,
            unblock_all,
            has_pin,
            set_pin,
            change_pin,
            reset_pin,
        ])
        .run(tauri::generate_context!())
        .expect("Errore avvio applicazione Tauri");
}
