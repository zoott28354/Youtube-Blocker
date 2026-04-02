#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod browsers;
mod config;
mod firewall;
mod hosts;

use auth::{hash_pin, verify_pin};
use browsers::{are_policies_active, disable_browser_doh, enable_browser_doh, is_browser_policy_supported};
use config::{config_path, load_config, save_config, AppConfig, BlockList};
use firewall::{add_firewall_rules, are_rules_active, is_firewall_supported, remove_firewall_rules};
use hosts::{block_sites, has_block_section, is_blocked, unblock_sites};
use std::process::Command;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
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

#[cfg(target_os = "macos")]
fn is_admin() -> bool {
    Command::new("id")
        .arg("-u")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
}

#[cfg(target_os = "macos")]
fn relaunch_as_admin() {
    fn shell_escape(value: &str) -> String {
        value.replace('\'', "'\"'\"'")
    }

    let exe = std::env::current_exe().expect("impossibile trovare eseguibile");
    let command = format!("'{}'", shell_escape(&exe.to_string_lossy()));
    Command::new("osascript")
        .args([
            "-e",
            &format!(
                "do shell script \"{}\" with administrator privileges",
                command
            ),
        ])
        .spawn()
        .expect("impossibile rilanciare con privilegi admin");
}

struct AppState(Mutex<AppConfig>);

#[derive(serde::Serialize)]
struct BlockStatus {
    os_name: &'static str,
    hosts_blocked: bool,
    firewall_supported: bool,
    firewall_active: bool,
    browser_policy_supported: bool,
    browser_policy: bool,
    block_doh_enabled: bool,
    active_lists_count: usize,
    active_list_names: Vec<String>,
}

// --- Helper: unione siti dalle liste ---

/// Siti delle sole liste attive — usato per block_all e get_status.
fn active_sites(cfg: &AppConfig) -> Vec<String> {
    cfg.lists
        .iter()
        .filter(|l| l.active)
        .flat_map(|l| l.sites.iter().cloned())
        .collect()
}

/// Siti di tutte le liste (attive e non) — usato per unblock_all e cleanup.
fn all_sites(cfg: &AppConfig) -> Vec<String> {
    cfg.lists
        .iter()
        .flat_map(|l| l.sites.iter().cloned())
        .collect()
}

fn is_protection_active_for_config(cfg: &AppConfig) -> Result<bool, String> {
    let sites = active_sites(cfg);
    if sites.is_empty() {
        return Ok(false);
    }

    let hosts_blocked = is_blocked(&sites).map_err(|e| e.to_string())?;
    if !hosts_blocked {
        return Ok(false);
    }

    if !cfg.block_doh {
        return Ok(true);
    }

    let firewall_ok = !is_firewall_supported() || are_rules_active();
    let browser_ok = !is_browser_policy_supported() || are_policies_active();
    Ok(firewall_ok && browser_ok)
}

fn clear_active_lists(cfg: &mut AppConfig) {
    for list in &mut cfg.lists {
        list.active = false;
    }
}

// --- Normalizzazione dominio ---

/// Prefissi sottodominio comuni (locale, mobile, www).
const SUBDOMAIN_PREFIXES: &[&str] = &[
    "www", "m",
    "it", "en", "fr", "de", "es", "pt", "nl", "ru", "pl", "tr",
    "ja", "ko", "zh", "ar", "hi", "th", "vi", "id",
    "sv", "da", "no", "fi", "cs", "el", "ro", "hu", "bg", "uk", "hr",
];

/// Normalizza l'input e restituisce root + varianti (www, m, locale).
fn expand_domain(input: &str) -> Vec<String> {
    let domain = input
        .trim()
        .to_lowercase()
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or("")
        .to_string();

    // Rimuove qualsiasi prefisso noto per ottenere il root
    let mut root = domain.as_str();
    for prefix in SUBDOMAIN_PREFIXES {
        if let Some(rest) = root.strip_prefix(prefix) {
            if let Some(rest) = rest.strip_prefix('.') {
                root = rest;
                break;
            }
        }
    }

    if root.is_empty() || !root.contains('.') {
        return vec![];
    }

    let mut variants = vec![root.to_string()];
    for prefix in SUBDOMAIN_PREFIXES {
        variants.push(format!("{}.{}", prefix, root));
    }
    variants
}

// --- Comandi Tauri: stato ---

#[tauri::command]
fn get_status(state: State<AppState>) -> Result<BlockStatus, String> {
    let cfg = state.0.lock().unwrap();
    let sites = active_sites(&cfg);
    let active_list_names: Vec<String> = cfg
        .lists
        .iter()
        .filter(|l| l.active)
        .map(|l| l.name.clone())
        .collect();
    let hosts_blocked = if sites.is_empty() {
        has_block_section().map_err(|e| e.to_string())?
    } else {
        is_blocked(&sites).map_err(|e| e.to_string())?
    };
    let firewall_supported = is_firewall_supported();
    let browser_policy_supported = is_browser_policy_supported();
    let firewall_active = if cfg.block_doh && firewall_supported {
        are_rules_active()
    } else {
        false
    };
    let browser_policy = if cfg.block_doh && browser_policy_supported {
        are_policies_active()
    } else {
        false
    };
    Ok(BlockStatus {
        os_name: std::env::consts::OS,
        hosts_blocked,
        firewall_supported,
        firewall_active,
        browser_policy_supported,
        browser_policy,
        block_doh_enabled: cfg.block_doh,
        active_lists_count: active_list_names.len(),
        active_list_names,
    })
}

#[tauri::command]
fn get_sites(state: State<AppState>) -> Vec<String> {
    active_sites(&state.0.lock().unwrap())
}

// --- Comandi Tauri: blocco ---

#[tauri::command]
fn block_all(state: State<AppState>) -> Result<(), String> {
    let cfg = state.0.lock().unwrap();
    let sites = active_sites(&cfg);
    if sites.is_empty() {
        return Err("Nessuna lista attiva".into());
    }
    block_sites(&sites).map_err(|e| e.to_string())?;
    if cfg.block_doh && is_firewall_supported() {
        add_firewall_rules().map_err(|e| e.to_string())?;
    }
    if cfg.block_doh && is_browser_policy_supported() {
        disable_browser_doh();
    }
    Ok(())
}

#[tauri::command]
fn unblock_all(pin: String, state: State<AppState>) -> Result<(), String> {
    let mut cfg = state.0.lock().unwrap();
    match &cfg.pin_hash {
        Some(hash) => verify_pin(&pin, hash).map_err(|e| e.to_string())?,
        None => return Err("Nessun PIN configurato".into()),
    }
    let sites = all_sites(&cfg);
    unblock_sites(&sites).map_err(|e| e.to_string())?;
    if cfg.block_doh && is_firewall_supported() {
        remove_firewall_rules().map_err(|e| e.to_string())?;
    }
    if cfg.block_doh && is_browser_policy_supported() {
        enable_browser_doh();
    }
    clear_active_lists(&mut cfg);
    save_config(&cfg).map_err(|e| e.to_string())?;
    Ok(())
}

// --- Comandi Tauri: liste ---

#[tauri::command]
fn get_lists(state: State<AppState>) -> Vec<BlockList> {
    state.0.lock().unwrap().lists.clone()
}

#[tauri::command]
fn create_list(name: String, state: State<AppState>) -> Result<BlockList, String> {
    let id = format!(
        "custom-{:x}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    let list = BlockList {
        id,
        name,
        sites: vec![],
        active: false,
        builtin: false,
    };
    let mut cfg = state.0.lock().unwrap();
    cfg.lists.push(list.clone());
    save_config(&cfg).map_err(|e| e.to_string())?;
    Ok(list)
}

#[tauri::command]
fn update_list(
    id: String,
    name: String,
    sites: Vec<String>,
    state: State<AppState>,
) -> Result<(), String> {
    let mut cfg = state.0.lock().unwrap();
    if let Some(list) = cfg.lists.iter_mut().find(|l| l.id == id) {
        list.name = name;
        list.sites = sites;
    }
    save_config(&cfg).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_list(id: String, state: State<AppState>) -> Result<(), String> {
    let mut cfg = state.0.lock().unwrap();
    cfg.lists.retain(|l| l.id != id);
    save_config(&cfg).map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_list(id: String, active: bool, state: State<AppState>) -> Result<(), String> {
    let mut cfg = state.0.lock().unwrap();
    if let Some(list) = cfg.lists.iter_mut().find(|l| l.id == id) {
        list.active = active;
    }
    save_config(&cfg).map_err(|e| e.to_string())?;

    // Se i siti erano già bloccati, aggiorna i siti bloccati nel file hosts
    let sites = active_sites(&cfg);
    let currently_blocked = is_blocked(&all_sites(&cfg)).unwrap_or(false);
    if currently_blocked {
        block_sites(&sites).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn add_site_to_list(list_id: String, domain: String, state: State<AppState>) -> Result<(), String> {
    let variants = expand_domain(&domain);
    if variants.is_empty() {
        return Err("Dominio non valido".into());
    }
    let mut cfg = state.0.lock().unwrap();
    if let Some(list) = cfg.lists.iter_mut().find(|l| l.id == list_id) {
        for variant in variants {
            if !list.sites.contains(&variant) {
                list.sites.push(variant);
            }
        }
    }
    save_config(&cfg).map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_site_from_list(
    list_id: String,
    domain: String,
    state: State<AppState>,
) -> Result<(), String> {
    let to_remove: std::collections::HashSet<String> =
        expand_domain(&domain).into_iter().collect();
    let mut cfg = state.0.lock().unwrap();
    if let Some(list) = cfg.lists.iter_mut().find(|l| l.id == list_id) {
        list.sites.retain(|s| !to_remove.contains(s));
    }
    save_config(&cfg).map_err(|e| e.to_string())
}

// --- Comandi Tauri: PIN ---

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
fn check_pin(pin: String, state: State<AppState>) -> Result<(), String> {
    let cfg = state.0.lock().unwrap();
    match &cfg.pin_hash {
        Some(hash) => verify_pin(&pin, hash).map_err(|e| e.to_string()),
        None => Err("Nessun PIN impostato".into()),
    }
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

// --- Comandi Tauri: utility ---

#[tauri::command]
fn open_url(url: String) {
    #[cfg(target_os = "windows")]
    let _ = Command::new("cmd").args(["/c", "start", "", &url]).spawn();
    #[cfg(target_os = "macos")]
    let _ = Command::new("open").arg(&url).spawn();
    #[cfg(target_os = "linux")]
    let _ = Command::new("xdg-open").arg(&url).spawn();
}

#[tauri::command]
fn get_app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// --- Modalità cleanup (chiamata da NSIS con --cleanup) ---

// Rimuove hosts, firewall, policy browser e config SENZA aprire la GUI.
fn run_cleanup() {
    let cfg = load_config().unwrap_or_default();
    let _ = unblock_sites(&all_sites(&cfg));
    let _ = remove_firewall_rules();
    enable_browser_doh();
    if let Ok(path) = config_path() {
        let _ = std::fs::remove_file(&path);
        if let Some(parent) = path.parent() {
            let _ = std::fs::remove_dir(parent);
        }
    }
}

fn main() {
    // Intercetta --cleanup PRIMA di qualsiasi inizializzazione Tauri.
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(|s| s.as_str()) == Some("--cleanup") {
        run_cleanup();
        return;
    }

    #[cfg(target_os = "windows")]
    if !is_admin() {
        relaunch_as_admin();
        return;
    }

    #[cfg(target_os = "macos")]
    if !is_admin() {
        relaunch_as_admin();
        return;
    }

    let mut config = load_config().unwrap_or_default();
    if !is_protection_active_for_config(&config).unwrap_or(false) {
        let had_active_lists = config.lists.iter().any(|list| list.active);
        if had_active_lists {
            clear_active_lists(&mut config);
            let _ = save_config(&config);
        }
    }

    tauri::Builder::default()
        .manage(AppState(Mutex::new(config)))
        .invoke_handler(tauri::generate_handler![
            get_status,
            get_sites,
            get_lists,
            create_list,
            update_list,
            delete_list,
            toggle_list,
            add_site_to_list,
            remove_site_from_list,
            block_all,
            unblock_all,
            has_pin,
            set_pin,
            change_pin,
            reset_pin,
            check_pin,
            open_url,
            get_app_version,
        ])
        .run(tauri::generate_context!())
        .expect("Errore avvio applicazione Tauri");
}
