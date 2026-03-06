# SiteBlocker — Contesto per Claude

## Cosa fa questo progetto
App desktop Tauri 2 per bloccare siti a livello di sistema su Windows.
Usecase principale: genitore blocca YouTube (e altri siti) per un figlio di 11 anni.
Blocca tramite file `hosts` + regole Windows Firewall anti-DoH.
PIN argon2id richiesto per sbloccare.

## Stack
- Tauri 2.x + React + TypeScript + Tailwind CSS
- Rust backend in `site-blocker/src-tauri/src/`

## File chiave

| File | Responsabilità |
|---|---|
| `src-tauri/src/main.rs` | Comandi Tauri, admin check runtime, AppState (Mutex) |
| `src-tauri/src/config.rs` | AppConfig, load/save `%LOCALAPPDATA%\SiteBlocker\config.json` |
| `src-tauri/src/auth.rs` | hash_pin / verify_pin con argon2id |
| `src-tauri/src/hosts.rs` | Lettura/scrittura file hosts + flush DNS multipiattaforma |
| `src-tauri/src/firewall.rs` | Regole netsh outbound TCP verso DoH su porte 443 e 853 |
| `src/hooks/useBlocker.ts` | Stato React centralizzato, tutti gli invoke Tauri |
| `src/App.tsx` | Routing tab, flusso primo avvio (setup PIN) |

## Decisioni architetturali importanti

### Admin elevation
Non usiamo manifest UAC (non supportato da tauri-build 2.5.x in tauri.conf.json).
Admin check a runtime: `net session` su Windows. Se non admin → rilancio con
`Start-Process -Verb RunAs` via PowerShell nascosto (CREATE_NO_WINDOW).

### Hosts file
- Marker: `# SiteBlocker <domain>` identifica le righe nostre
- Rimozione: match ESATTO sulle righe scritte da noi (HashSet), NON match parziale
  su "127.0.0.1" + dominio (bug storico che svuotava il file hosts)
- Line endings: `\r\n` su Windows, `\n` su Mac/Linux (cfg! compile-time)
- flush_dns() chiamato dopo ogni write

### Firewall
- Nome regole: `SiteBlocker_DoH_<ip>_p<porta>` (deterministico)
- Remove-before-add: idempotente, sicuro chiamare più volte
- Solo TCP (DoH su QUIC/UDP porta 443 non bloccato — possibile v2)

### PIN
- argon2id, salt OsRng, hash self-describing (include params + salt nel JSON)
- Blocco NON richiede PIN. Solo sblocco richiede PIN.
- Minimo 4 caratteri (validato sia Rust che React)

### Config
- `pin_hash: Option<String>` — None = primo avvio, mostra setup PIN
- `block_doh: bool` — default true, toggle in Settings (da implementare)

## Comandi Tauri disponibili
```
get_status()             → { hosts_blocked, firewall_active }
get_sites()              → Vec<String>
add_site(domain)         → Result<()>
remove_site(domain)      → Result<()>
block_all()              → Result<()>   // no PIN
unblock_all(pin)         → Result<()>   // PIN obbligatorio
has_pin()                → bool
set_pin(pin)             → Result<()>
change_pin(old, new)     → Result<()>
```

## Roadmap / Feature future
- **Game Timer**: integrazione con un timer di gioco che blocca/sblocca automaticamente
- **Schedule**: blocco automatico per fascia oraria (es. 15:00-18:00 studio)
- **System tray**: toggle rapido senza aprire la finestra principale
- **Profili**: set di regole nominati (Studio, Lavoro, Weekend)
- **Mac/Linux**: hosts_path() già cross-platform; manca firewall (pfctl / iptables) e admin elevation (pkexec / sudo)
- **Toggle block_doh**: UI per abilitare/disabilitare le regole firewall separatamente dagli hosts

## Gotcha e bug noti risolti
- `tauri_build::Builder` non esiste in v2 → usare `tauri_build::build()`
- `requestedExecutionLevel` in tauri.conf.json non supportato da tauri-build 2.5.x
- `winres` in build.rs conflicta con tauri-build → rimosso
- ICO icon richiesta per build su Windows → generata con System.Drawing
- `ttk.Label` in Python non supporta foreground con tema vista → usare `tk.Label`
- PowerShell: `Is-YoutubeBlocked` usava `-SimpleMatch` con pattern regex → rimosso
- PowerShell: `Unblock-Youtube` usava `-not` su array (ForEach-Object) → Where-Object
- PowerShell: `MessageBox.Show` parametri invertiti (buttons/icon) → enum tipizzati

## Come fare la build
```bash
cd site-blocker
npm install
npm run tauri dev   # terminale come Admin
npm run tauri build
```
