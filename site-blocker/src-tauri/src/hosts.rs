use std::fs;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

const BLOCK_MARKER: &str = "# SiteBlocker";

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
        // systemd-resolved (Ubuntu, Fedora, Arch...)
        let _ = Command::new("systemd-resolve").args(["--flush-caches"]).output();
        // fallback nscd
        let _ = Command::new("service").args(["nscd", "restart"]).output();
    }
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

/// Aggiunge le voci di blocco per i siti non ancora presenti.
pub fn block_sites(sites: &[String]) -> Result<(), HostsError> {
    let path = hosts_path();
    let content = fs::read_to_string(&path)?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    for domain in sites {
        let already_present = lines.iter().any(|l| {
            let t = l.trim();
            !t.starts_with('#') && t.contains("127.0.0.1") && t.contains(domain.as_str())
        });
        if !already_present {
            lines.push(format!("{} {}", BLOCK_MARKER, domain));
            lines.push(format!("127.0.0.1 {}", domain));
        }
    }

    let eol = if cfg!(target_os = "windows") { "\r\n" } else { "\n" };
    fs::write(&path, lines.join(eol) + eol)?;
    flush_dns();
    Ok(())
}

/// Rimuove tutte le voci gestite da SiteBlocker dal file hosts.
/// Usa match esatto per evitare di rimuovere righe non aggiunte da noi.
pub fn unblock_sites(sites: &[String]) -> Result<(), HostsError> {
    let path = hosts_path();
    let content = fs::read_to_string(&path)?;

    // Insieme esatto delle righe che abbiamo scritto noi
    let our_lines: std::collections::HashSet<String> = sites
        .iter()
        .flat_map(|domain| {
            [
                format!("{} {}", BLOCK_MARKER, domain),
                format!("127.0.0.1 {}", domain),
            ]
        })
        .collect();

    let filtered: Vec<&str> = content
        .lines()
        .filter(|line| !our_lines.contains(line.trim()))
        .collect();

    let eol = if cfg!(target_os = "windows") { "\r\n" } else { "\n" };
    fs::write(&path, filtered.join(eol) + eol)?;
    flush_dns();
    Ok(())
}
