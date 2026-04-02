use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FirewallError {
    #[error("Comando fallito per {0}: {1}")]
    CommandFailed(String, String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

const DOH_SERVERS: &[&str] = &[
    "1.1.1.1", // Cloudflare
    "1.0.0.1", // Cloudflare secondary
    "8.8.8.8", // Google
    "8.8.4.4", // Google secondary
    "9.9.9.9", // Quad9
];
const DOH_PORTS: &[&str] = &["443", "853"];

#[cfg(target_os = "windows")]
const RULE_PREFIX: &str = "YouTubeBlocker_DoH_";

#[cfg(target_os = "macos")]
const PF_ANCHOR_NAME: &str = "com.youtubeblocker";
#[cfg(target_os = "macos")]
const PF_ANCHOR_FILE: &str = "/etc/pf.anchors/com.youtubeblocker";
#[cfg(target_os = "macos")]
const PF_CONF: &str = "/etc/pf.conf";

pub fn is_firewall_supported() -> bool {
    cfg!(target_os = "windows") || cfg!(target_os = "macos")
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

    #[cfg(target_os = "macos")]
    {
        return add_firewall_rules_macos();
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
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

    #[cfg(target_os = "macos")]
    {
        return remove_firewall_rules_macos();
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
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

    #[cfg(target_os = "macos")]
    {
        return are_rules_active_macos();
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    false
}

// ─── macOS (pfctl) ──────────────────────────────────────────────────────────

#[cfg(target_os = "macos")]
fn generate_pf_rules() -> String {
    let mut rules = String::new();
    for &ip in DOH_SERVERS {
        rules.push_str(&format!(
            "block drop out quick proto tcp from any to {} port {{ {} }}\n",
            ip,
            DOH_PORTS.join(", ")
        ));
    }
    rules
}

#[cfg(target_os = "macos")]
fn ensure_anchor_in_pf_conf() -> Result<(), FirewallError> {
    let content = std::fs::read_to_string(PF_CONF)?;

    let anchor_line = format!("anchor \"{}\"", PF_ANCHOR_NAME);
    let load_line = format!(
        "load anchor \"{}\" from \"{}\"",
        PF_ANCHOR_NAME, PF_ANCHOR_FILE
    );

    let needs_anchor = !content.contains(&anchor_line);
    let needs_load = !content.contains(&load_line);

    if needs_anchor || needs_load {
        let mut new_content = content.clone();
        if !new_content.ends_with('\n') {
            new_content.push('\n');
        }
        if needs_anchor {
            new_content.push_str(&format!("{}\n", anchor_line));
        }
        if needs_load {
            new_content.push_str(&format!("{}\n", load_line));
        }
        std::fs::write(PF_CONF, new_content)?;
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn remove_anchor_from_pf_conf() -> Result<(), FirewallError> {
    let content = std::fs::read_to_string(PF_CONF)?;
    let filtered: Vec<&str> = content
        .lines()
        .filter(|line| !line.contains(PF_ANCHOR_NAME))
        .collect();
    std::fs::write(PF_CONF, filtered.join("\n") + "\n")?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn add_firewall_rules_macos() -> Result<(), FirewallError> {
    // 1. Scrivi le regole nel file anchor
    let _ = std::fs::create_dir_all("/etc/pf.anchors");
    std::fs::write(PF_ANCHOR_FILE, generate_pf_rules())?;

    // 2. Assicurati che pf.conf referenzi il nostro anchor
    ensure_anchor_in_pf_conf()?;

    // 3. Carica le regole nell'anchor
    let out = Command::new("pfctl")
        .args(["-a", PF_ANCHOR_NAME, "-f", PF_ANCHOR_FILE])
        .output()?;
    if !out.status.success() {
        return Err(FirewallError::CommandFailed(
            "pfctl load".into(),
            String::from_utf8_lossy(&out.stderr).to_string(),
        ));
    }

    // 4. Abilita pf (ignora errore "already enabled")
    let _ = Command::new("pfctl").arg("-e").output();

    Ok(())
}

#[cfg(target_os = "macos")]
fn remove_firewall_rules_macos() -> Result<(), FirewallError> {
    // 1. Svuota le regole dell'anchor
    let _ = Command::new("pfctl")
        .args(["-a", PF_ANCHOR_NAME, "-F", "all"])
        .output();

    // 2. Rimuovi il file anchor
    let _ = std::fs::remove_file(PF_ANCHOR_FILE);

    // 3. Rimuovi i riferimenti da pf.conf
    let _ = remove_anchor_from_pf_conf();

    Ok(())
}

#[cfg(target_os = "macos")]
fn are_rules_active_macos() -> bool {
    let output = Command::new("pfctl")
        .args(["-a", PF_ANCHOR_NAME, "-sr"])
        .output();
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.contains("block")
        }
        Err(_) => false,
    }
}
