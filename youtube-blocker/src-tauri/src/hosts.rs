use std::fs;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

const SECTION_START: &str = "## YouTubeBlocker - Start ##";
const SECTION_END:   &str = "## YouTubeBlocker - End ##";
const BLOCK_MARKER:  &str = "# YouTubeBlocker"; // mantenuto per retrocompatibilità

#[derive(Debug, Error)]
pub enum HostsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[cfg(target_os = "macos")]
    #[error("Comando privilegiato fallito: {0}")]
    CommandFailed(String),
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

#[cfg(target_os = "macos")]
fn shell_escape(value: &str) -> String {
    value.replace('\'', "'\"'\"'")
}

#[cfg(target_os = "macos")]
fn write_hosts_with_privileges(content: &str) -> Result<(), HostsError> {
    let path = hosts_path();
    let temp_path = std::env::temp_dir().join("youtube-blocker-hosts.tmp");
    fs::write(&temp_path, content)?;

    let command = format!(
        "cp '{}' '{}' && dscacheutil -flushcache && killall -HUP mDNSResponder",
        shell_escape(&temp_path.to_string_lossy()),
        shell_escape(&path.to_string_lossy()),
    );

    let output = Command::new("osascript")
        .args([
            "-e",
            &format!(
                "do shell script \"{}\" with administrator privileges",
                command.replace('\\', "\\\\").replace('"', "\\\"")
            ),
        ])
        .output()?;

    let _ = fs::remove_file(&temp_path);

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let details = if !stderr.is_empty() { stderr } else { stdout };
        Err(HostsError::CommandFailed(details))
    }
}

fn write_hosts_content(content: &str) -> Result<(), HostsError> {
    #[cfg(target_os = "macos")]
    {
        return write_hosts_with_privileges(content);
    }

    #[cfg(not(target_os = "macos"))]
    {
        fs::write(hosts_path(), content)?;
        flush_dns();
        Ok(())
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

fn blocked_domains_from_content(content: &str) -> std::collections::HashSet<String> {
    let mut blocked = std::collections::HashSet::new();
    let mut in_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == SECTION_START {
            in_section = true;
            continue;
        }
        if trimmed == SECTION_END {
            in_section = false;
            continue;
        }

        if in_section {
            let mut parts = trimmed.split_whitespace();
            if let (Some(ip), Some(domain), None) = (parts.next(), parts.next(), parts.next()) {
                if ip == "127.0.0.1" {
                    blocked.insert(domain.to_string());
                }
            }
            continue;
        }

        if let Some(domain) = trimmed.strip_prefix("127.0.0.1 ") {
            // Retrocompatibilita' con il vecchio formato senza sezione dedicata.
            if !domain.contains(char::is_whitespace) && !domain.starts_with('#') {
                blocked.insert(domain.to_string());
            }
        }
    }

    blocked
}

/// True solo se TUTTI i domini hanno una voce 127.0.0.1 nel file hosts.
pub fn is_blocked(sites: &[String]) -> Result<bool, HostsError> {
    if sites.is_empty() {
        return Ok(false);
    }

    let content = fs::read_to_string(hosts_path())?;
    let blocked = blocked_domains_from_content(&content);
    Ok(sites.iter().all(|domain| blocked.contains(domain)))
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
    write_hosts_content(&(base.join(eol) + eol))
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
    write_hosts_content(&(filtered.join(eol) + eol))
}
