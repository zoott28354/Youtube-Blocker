/// Disabilita/abilita il DNS-over-HTTPS nei principali browser via Group Policy.
///
/// Windows: chiave registro HKLM\SOFTWARE\Policies\...\DnsOverHttpsMode = "off"
/// macOS:   plist in /Library/Managed Preferences/<bundle_id>.plist
/// Firefox: crea/rimuove distribution/policies.json nella cartella di installazione.
///
/// Tutte le operazioni sono best-effort (il browser potrebbe non essere installato).
use std::path::{Path, PathBuf};
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

// ─── Costanti macOS ─────────────────────────────────────────────────────────

#[cfg(target_os = "macos")]
const CHROMIUM_MANAGED_PREF_BUNDLES: &[&str] = &[
    "com.google.Chrome",
    "com.microsoft.Edge",
    "com.brave.Browser",
    "com.vivaldi.Vivaldi",
    "com.operasoftware.Opera",
    "org.chromium.Chromium",
];

#[cfg(target_os = "macos")]
const MANAGED_PREFS_DIR: &str = "/Library/Managed Preferences";

#[cfg(target_os = "macos")]
const CHROMIUM_PLIST_CONTENT: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>DnsOverHttpsMode</key>
    <string>off</string>
</dict>
</plist>"#;

#[cfg(target_os = "macos")]
const FIREFOX_INSTALL_DIRS: &[&str] = &[
    "/Applications/Firefox.app/Contents/Resources",
];

// ─── Costanti condivise ─────────────────────────────────────────────────────

/// Marker nel JSON per riconoscere il file creato da noi.
const YOUTUBEBLOCKER_MARKER: &str = "\"_youtubeblocker\":true";

const FIREFOX_POLICY_JSON: &str =
    r#"{"policies":{"DNSOverHTTPS":{"Enabled":false,"Locked":true}},"_youtubeblocker":true}"#;

// ─── API pubblica ───────────────────────────────────────────────────────────

pub fn is_browser_policy_supported() -> bool {
    cfg!(target_os = "windows") || cfg!(target_os = "macos")
}

/// Verifica se almeno una policy browser è attiva (Chrome/Edge o Firefox).
pub fn are_policies_active() -> bool {
    #[cfg(target_os = "windows")]
    {
        // Controlla la chiave Chrome come campione
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
        // Controlla Firefox policies.json
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

    #[cfg(target_os = "macos")]
    {
        // Controlla Chrome come campione
        let plist_path = Path::new(MANAGED_PREFS_DIR)
            .join(format!("{}.plist", CHROMIUM_MANAGED_PREF_BUNDLES[0]));
        if let Ok(content) = std::fs::read_to_string(&plist_path) {
            if content.contains("DnsOverHttpsMode") {
                return true;
            }
        }
        // Controlla Firefox policies.json
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

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    false
}

/// Disabilita DoH in Chrome, Edge, Brave, Firefox, ecc.
/// Chiamato durante il blocco (se block_doh è abilitato).
pub fn disable_browser_doh() {
    #[cfg(target_os = "windows")]
    {
        set_chromium_doh_windows(false);
        set_firefox_doh(false);
    }
    #[cfg(target_os = "macos")]
    {
        set_chromium_doh_macos(false);
        set_firefox_doh(false);
    }
}

/// Ripristina le impostazioni DoH dei browser.
/// Chiamato durante lo sblocco.
pub fn enable_browser_doh() {
    #[cfg(target_os = "windows")]
    {
        set_chromium_doh_windows(true);
        set_firefox_doh(true);
    }
    #[cfg(target_os = "macos")]
    {
        set_chromium_doh_macos(true);
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

// ─── Chromium macOS (managed preferences plist) ─────────────────────────────

#[cfg(target_os = "macos")]
fn set_chromium_doh_macos(allow: bool) {
    let managed_dir = Path::new(MANAGED_PREFS_DIR);
    let _ = std::fs::create_dir_all(managed_dir);

    for &bundle in CHROMIUM_MANAGED_PREF_BUNDLES {
        let plist_path = managed_dir.join(format!("{}.plist", bundle));
        if allow {
            // Rimuovi solo se contiene il nostro contenuto
            if let Ok(content) = std::fs::read_to_string(&plist_path) {
                if content.contains("DnsOverHttpsMode") {
                    let _ = std::fs::remove_file(&plist_path);
                }
            }
        } else {
            let _ = std::fs::write(&plist_path, CHROMIUM_PLIST_CONTENT);
        }
    }
}

// ─── Firefox (policies.json, cross-platform) ───────────────────────────────

fn firefox_dist_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    let dirs = FIREFOX_INSTALL_DIRS;
    #[cfg(target_os = "macos")]
    let dirs = FIREFOX_INSTALL_DIRS;
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
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
        return; // Firefox non installato
    };

    let policy_path = dist_dir.join("policies.json");
    let backup_path = dist_dir.join("policies.json.siteblocker.bak");

    if allow {
        // Ripristina: rimuovi il nostro file e, se esiste, rimetti il backup
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
        // Blocca: crea la dir, fai backup se esiste un policies.json pre-esistente
        let _ = std::fs::create_dir_all(&dist_dir);
        if policy_path.exists() {
            let content = std::fs::read_to_string(&policy_path).unwrap_or_default();
            // Non sovrascrivere un backup già esistente; non toccare file già nostri
            if !content.contains(YOUTUBEBLOCKER_MARKER) && !backup_path.exists() {
                let _ = std::fs::copy(&policy_path, &backup_path);
            }
        }
        let _ = std::fs::write(&policy_path, FIREFOX_POLICY_JSON);
    }
}
