use std::fs;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

const SECTION_START: &str = "## YouTubeBlocker - Start ##";
const SECTION_END:   &str = "## YouTubeBlocker - End ##";
const BLOCK_MARKER:  &str = "# SiteBlocker"; // mantenuto per retrocompatibilità

#[derive(Debug, Error)]
pub enum HostsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn hosts_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let windir = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".into());
        PathBuf::from(windir).join("System32").join("drivers").join("etc").join("hosts")
    }
    #[cfg(not(target_os = "windows"))]
    {
        PathBuf::from("/etc/hosts")
    }
}

fn flush_dns() {
    #[cfg(target_os = "windows")]
    { let _ = Command::new("ipconfig").args(["/flushdns"]).output(); }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("dscacheutil").args(["-flushcache"]).output();
        let _ = Command::new("killall").args(["-HUP", "mDNSResponder"]).output();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("systemd-resolve").args(["--flush-caches"]).output();
        let _ = Command::new("service").args(["nscd", "restart"]).output();
    }
}

/// Rimuove la sezione YouTubeBlocker (marker inclusi) e le righe vuote che la precedono.
fn remove_our_section(lines: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut in_section = false;
    for line in lines {
        if line.trim() == SECTION_START {
            in_section = true;
            // Rimuove le righe vuote aggiunte prima della sezione
            while result.last().map(|l: &String| l.trim().is_empty()).unwrap_or(false) {
                result.pop();
            }
            continue;
        }
        if line.trim() == SECTION_END {
            in_section = false;
            continue;
        }
        if !in_section {
            result.push(line);
        }
    }
    result
}

/// True solo se TUTTI i domini hanno una voce 127.0.0.1 nel file hosts.
pub fn is_blocked(sites: &[String]) -> Result<bool, HostsError> {
    let content = fs::read_to_string(hosts_path())?;
    Ok(sites.iter().all(|domain| {
        content.lines().any(|line| {
            let t = line.trim();
            !t.starts_with('#')
                && t.contains("127.0.0.1")
                && t.contains(domain.as_str())
        })
    }))
}

/// Aggiunge (o riscrive) la sezione YouTubeBlocker nel file hosts.
pub fn block_sites(sites: &[String]) -> Result<(), HostsError> {
    let path = hosts_path();
    let content = fs::read_to_string(&path)?;
    let lines: Vec<String> = content.lines().map(String::from).collect();

    // Rimuove sezione esistente (idempotente)
    let mut base = remove_our_section(lines);

    // Fallback retrocompatibilità: rimuove righe vecchio formato senza sezione
    let legacy: std::collections::HashSet<String> = sites
        .iter()
        .flat_map(|d| [format!("{} {}", BLOCK_MARKER, d), format!("127.0.0.1 {}", d)])
        .collect();
    base.retain(|l| !legacy.contains(l.trim()));

    // Aggiunge la nuova sezione
    base.push(String::new());
    base.push(String::new());
    base.push(SECTION_START.to_string());
    for domain in sites {
        base.push(format!("127.0.0.1 {}", domain));
    }
    base.push(SECTION_END.to_string());

    let eol = if cfg!(target_os = "windows") { "\r\n" } else { "\n" };
    fs::write(&path, base.join(eol) + eol)?;
    flush_dns();
    Ok(())
}

/// Rimuove la sezione YouTubeBlocker dal file hosts.
pub fn unblock_sites(sites: &[String]) -> Result<(), HostsError> {
    let path = hosts_path();
    let content = fs::read_to_string(&path)?;
    let lines: Vec<String> = content.lines().map(String::from).collect();

    // Rimuove sezione (nuovo formato)
    let mut filtered = remove_our_section(lines);

    // Fallback retrocompatibilità: rimuove righe vecchio formato
    let legacy: std::collections::HashSet<String> = sites
        .iter()
        .flat_map(|d| [format!("{} {}", BLOCK_MARKER, d), format!("127.0.0.1 {}", d)])
        .collect();
    filtered.retain(|l| !legacy.contains(l.trim()));

    let eol = if cfg!(target_os = "windows") { "\r\n" } else { "\n" };
    fs::write(&path, filtered.join(eol) + eol)?;
    flush_dns();
    Ok(())
}
