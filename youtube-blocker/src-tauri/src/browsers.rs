/// Disabilita/abilita il DNS-over-HTTPS nei principali browser via Group Policy.
///
/// Chrome/Edge/Brave: chiave di registro HKLM\SOFTWARE\Policies\...\DnsOverHttpsMode = "off"
/// Firefox: crea/rimuove distribution/policies.json nella cartella di installazione.
///
/// Tutte le operazioni sono best-effort (il browser potrebbe non essere installato).
use std::path::{Path, PathBuf};
use std::process::Command;

const CHROMIUM_POLICY_KEYS: &[&str] = &[
    r"HKLM\SOFTWARE\Policies\Google\Chrome",
    r"HKLM\SOFTWARE\Policies\Microsoft\Edge",
    r"HKLM\SOFTWARE\Policies\BraveSoftware\Brave",
];

const FIREFOX_INSTALL_DIRS: &[&str] = &[
    r"C:\Program Files\Mozilla Firefox",
    r"C:\Program Files (x86)\Mozilla Firefox",
];

/// Marker nel JSON per riconoscere il file creato da noi.
const YOUTUBEBLOCKER_MARKER: &str = "\"_youtubeblocker\":true";

const FIREFOX_POLICY_JSON: &str =
    r#"{"policies":{"DNSOverHTTPS":{"Enabled":false,"Locked":true}},"_youtubeblocker":true}"#;

// ─── API pubblica ────────────────────────────────────────────────────────────

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
        false
    }
    #[cfg(not(target_os = "windows"))]
    false
}

/// Disabilita DoH in Chrome, Edge, Brave e Firefox.
/// Chiamato durante il blocco (se block_doh è abilitato).
pub fn disable_browser_doh() {
    #[cfg(target_os = "windows")]
    {
        set_chromium_doh(false);
        set_firefox_doh(false);
    }
}

/// Ripristina le impostazioni DoH dei browser.
/// Chiamato durante lo sblocco.
pub fn enable_browser_doh() {
    #[cfg(target_os = "windows")]
    {
        set_chromium_doh(true);
        set_firefox_doh(true);
    }
}

// ─── Chromium (Chrome / Edge / Brave) ───────────────────────────────────────

fn set_chromium_doh(allow: bool) {
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

// ─── Firefox ─────────────────────────────────────────────────────────────────

fn firefox_dist_dir() -> Option<PathBuf> {
    for &dir in FIREFOX_INSTALL_DIRS {
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
