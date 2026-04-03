/// Disabilita/abilita il DNS-over-HTTPS nei principali browser via Group Policy.
///
/// Windows: chiave registro HKLM\SOFTWARE\Policies\...\DnsOverHttpsMode = "off"
/// Firefox: crea/rimuove distribution/policies.json nella cartella di installazione.
///
/// Tutte le operazioni sono best-effort (il browser potrebbe non essere installato).
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
use std::process::Command;

// ─── Costanti Windows ───────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
const CHROMIUM_POLICY_KEYS: &[&str] = &[
    r"HKLM\SOFTWARE\Policies\Google\Chrome",
    r"HKLM\SOFTWARE\Policies\Microsoft\Edge",
    r"HKLM\SOFTWARE\Policies\BraveSoftware\Brave",
    r"HKLM\SOFTWARE\Policies\Vivaldi",
    r"HKLM\SOFTWARE\Policies\Opera Software\Opera",
    r"HKLM\SOFTWARE\Policies\Chromium",
];

#[cfg(target_os = "windows")]
const FIREFOX_INSTALL_DIRS: &[&str] = &[
    r"C:\Program Files\Mozilla Firefox",
    r"C:\Program Files (x86)\Mozilla Firefox",
];

// ─── Costanti condivise ─────────────────────────────────────────────────────

/// Marker nel JSON per riconoscere il file creato da noi.
const YOUTUBEBLOCKER_MARKER: &str = "\"_youtubeblocker\":true";

const FIREFOX_POLICY_JSON: &str =
    r#"{"policies":{"DNSOverHTTPS":{"Enabled":false,"Locked":true}},"_youtubeblocker":true}"#;

// ─── API pubblica ───────────────────────────────────────────────────────────

pub fn is_browser_policy_supported() -> bool {
    cfg!(target_os = "windows")
}

/// Verifica se almeno una policy browser è attiva (Chrome/Edge o Firefox).
pub fn are_policies_active() -> bool {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("reg")
            .args([
                "query",
                CHROMIUM_POLICY_KEYS[0],
                "/v",
                "DnsOverHttpsMode",
            ])
            .output();
        if let Ok(o) = output {
            if String::from_utf8_lossy(&o.stdout).contains("off") {
                return true;
            }
        }
        if let Some(dist_dir) = firefox_dist_dir() {
            let policy_path = dist_dir.join("policies.json");
            if let Ok(content) = std::fs::read_to_string(&policy_path) {
                if content.contains(YOUTUBEBLOCKER_MARKER) {
                    return true;
                }
            }
        }
        return false;
    }

    #[cfg(not(target_os = "windows"))]
    false
}

/// Disabilita DoH in Chrome, Edge, Brave, Firefox, ecc.
pub fn disable_browser_doh() {
    #[cfg(target_os = "windows")]
    {
        set_chromium_doh_windows(false);
        set_firefox_doh(false);
    }
}

/// Ripristina le impostazioni DoH dei browser.
pub fn enable_browser_doh() {
    #[cfg(target_os = "windows")]
    {
        set_chromium_doh_windows(true);
        set_firefox_doh(true);
    }
}

// ─── Chromium Windows (registry) ────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn set_chromium_doh_windows(allow: bool) {
    for &key in CHROMIUM_POLICY_KEYS {
        if allow {
            let _ = Command::new("reg")
                .args(["delete", key, "/v", "DnsOverHttpsMode", "/f"])
                .output();
        } else {
            let _ = Command::new("reg")
                .args([
                    "add", key,
                    "/v", "DnsOverHttpsMode",
                    "/t", "REG_SZ",
                    "/d", "off",
                    "/f",
                ])
                .output();
        }
    }
}

// ─── Firefox (policies.json, cross-platform) ───────────────────────────────

fn firefox_dist_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    let dirs = FIREFOX_INSTALL_DIRS;
    #[cfg(not(target_os = "windows"))]
    let dirs: &[&str] = &[];

    for &dir in dirs {
        let p = Path::new(dir);
        if p.exists() {
            return Some(p.join("distribution"));
        }
    }
    None
}

fn set_firefox_doh(allow: bool) {
    let Some(dist_dir) = firefox_dist_dir() else {
        return;
    };

    let policy_path = dist_dir.join("policies.json");
    let backup_path = dist_dir.join("policies.json.siteblocker.bak");

    if allow {
        if let Ok(content) = std::fs::read_to_string(&policy_path) {
            if content.contains(YOUTUBEBLOCKER_MARKER) {
                if backup_path.exists() {
                    let _ = std::fs::rename(&backup_path, &policy_path);
                } else {
                    let _ = std::fs::remove_file(&policy_path);
                }
            }
        }
    } else {
        let _ = std::fs::create_dir_all(&dist_dir);
        if policy_path.exists() {
            let content = std::fs::read_to_string(&policy_path).unwrap_or_default();
            if !content.contains(YOUTUBEBLOCKER_MARKER) && !backup_path.exists() {
                let _ = std::fs::copy(&policy_path, &backup_path);
            }
        }
        let _ = std::fs::write(&policy_path, FIREFOX_POLICY_JSON);
    }
}
