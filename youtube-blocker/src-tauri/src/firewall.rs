use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FirewallError {
    #[error("Comando fallito per {0}: {1}")]
    CommandFailed(String, String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(target_os = "windows")]
const DOH_SERVERS: &[&str] = &[
    "1.1.1.1", // Cloudflare
    "1.0.0.1", // Cloudflare secondary
    "8.8.8.8", // Google
    "8.8.4.4", // Google secondary
    "9.9.9.9", // Quad9
];
#[cfg(target_os = "windows")]
const DOH_PORTS: &[&str] = &["443", "853"];

#[cfg(target_os = "windows")]
const RULE_PREFIX: &str = "YouTubeBlocker_DoH_";

pub fn is_firewall_supported() -> bool {
    cfg!(target_os = "windows")
}

// ─── Windows (netsh) ────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn rule_name(ip: &str, port: &str) -> String {
    format!("{}{}_p{}", RULE_PREFIX, ip.replace('.', "_"), port)
}

#[cfg(target_os = "windows")]
fn delete_rule(name: &str) -> Result<(), FirewallError> {
    // Ignora exit code: la regola potrebbe non esistere
    Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "delete",
            "rule",
            &format!("name={}", name),
        ])
        .output()?;
    Ok(())
}

/// Aggiunge regole firewall outbound per bloccare i resolver DoH noti.
pub fn add_firewall_rules() -> Result<(), FirewallError> {
    #[cfg(target_os = "windows")]
    {
        for &ip in DOH_SERVERS {
            for &port in DOH_PORTS {
                let name = rule_name(ip, port);
                // Rimuove prima per evitare duplicati
                let _ = delete_rule(&name);
                let out = Command::new("netsh")
                    .args([
                        "advfirewall",
                        "firewall",
                        "add",
                        "rule",
                        &format!("name={}", name),
                        "dir=out",
                        "action=block",
                        "protocol=TCP",
                        &format!("remoteip={}", ip),
                        &format!("remoteport={}", port),
                        "enable=yes",
                        "profile=any",
                    ])
                    .output()?;
                if !out.status.success() {
                    return Err(FirewallError::CommandFailed(
                        name,
                        String::from_utf8_lossy(&out.stderr).to_string(),
                    ));
                }
            }
        }
        return Ok(());
    }

    #[cfg(not(target_os = "windows"))]
    Ok(())
}

/// Rimuove tutte le regole firewall YouTubeBlocker.
pub fn remove_firewall_rules() -> Result<(), FirewallError> {
    #[cfg(target_os = "windows")]
    {
        for &ip in DOH_SERVERS {
            for &port in DOH_PORTS {
                delete_rule(&rule_name(ip, port))?;
            }
        }
        return Ok(());
    }

    #[cfg(not(target_os = "windows"))]
    Ok(())
}

/// Verifica se le regole sono attive controllando una regola campione.
pub fn are_rules_active() -> bool {
    #[cfg(target_os = "windows")]
    {
        let name = rule_name("1.1.1.1", "443");
        let output = Command::new("netsh")
            .args([
                "advfirewall",
                "firewall",
                "show",
                "rule",
                &format!("name={}", name),
            ])
            .output();
        return match output {
            Ok(o) => String::from_utf8_lossy(&o.stdout).contains("Block"),
            Err(_) => false,
        };
    }

    #[cfg(not(target_os = "windows"))]
    false
}

