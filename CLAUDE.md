# YouTube Blocker — Contesto per Claude

## Cosa fa questo progetto
App desktop Tauri 2 per bloccare siti a livello di sistema su Windows.
Usecase principale: genitore blocca YouTube (e altri siti) per un figlio di 11 anni.
Tre livelli di blocco: file `hosts` + regole Windows Firewall anti-DoH + Group Policy browser.
PIN argon2id richiesto per aprire l'app e per sbloccare. Interfaccia bilingue IT/EN.

## Stack
- Tauri 2.x + React + TypeScript + Tailwind CSS
- Rust backend in `site-blocker/src-tauri/src/`

## Struttura repo
```
YouTube-Blocker/
├── scripts/              — bat file per setup/dev/build/bump_version
├── LICENSE               — MIT, Copyright 2025 zoott28354
├── README.md
├── CLAUDE.md
└── site-blocker/         — progetto Tauri
    ├── src-tauri/src/
    ├── src/
    └── public/
```

## Cartelle generate (NON nel git, eliminabili)
| Cartella | Come si rigenera |
|---|---|
| `site-blocker/node_modules/` | `setup.bat` / `npm install` |
| `site-blocker/dist/` | automatica durante build |
| `site-blocker/.vite/` | automatica |
| `site-blocker/src-tauri/target/` | automatica (lenta, ~20 min prima volta) |

## File chiave

| File | Responsabilità |
|---|---|
| `src-tauri/src/main.rs` | Comandi Tauri, admin check runtime, AppState (Mutex) |
| `src-tauri/src/config.rs` | AppConfig, load/save `%LOCALAPPDATA%\SiteBlocker\config.json` |
| `src-tauri/src/auth.rs` | hash_pin / verify_pin con argon2id |
| `src-tauri/src/hosts.rs` | Lettura/scrittura file hosts + flush DNS multipiattaforma |
| `src-tauri/src/firewall.rs` | Regole netsh outbound TCP verso DoH su porte 443 e 853 |
| `src-tauri/src/browsers.rs` | Group Policy DoH per Chrome/Edge/Brave (registry) e Firefox (policies.json) |
| `src/i18n.tsx` | Traduzioni IT/EN, LangProvider context, useI18n hook |
| `src/hooks/useBlocker.ts` | Stato React centralizzato, tutti gli invoke Tauri |
| `src/App.tsx` | Routing tab, session lock, setup PIN, toggle lingua |

## Decisioni architetturali importanti

### Admin elevation
Non usiamo manifest UAC (non supportato da tauri-build 2.5.x in tauri.conf.json).
Admin check a runtime: `net session` su Windows. Se non admin → rilancio con
`Start-Process -Verb RunAs` via PowerShell nascosto (CREATE_NO_WINDOW).

### Session lock (PIN all'apertura)
- All'apertura dell'app, se PIN è impostato, viene mostrata una schermata di verifica PIN
- Solo dopo autenticazione si accede all'interfaccia principale
- `isSessionUnlocked` in useBlocker.ts — false al mount, true dopo check_pin con successo
- Sblocco siti richiede ancora PIN (difesa in profondità)
- Reset PIN azzera anche la sessione corrente

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

### Browser DoH (browsers.rs)
- Chrome/Edge/Brave: chiave registry `HKLM\SOFTWARE\Policies\...\DnsOverHttpsMode = "off"`
- Firefox: crea `distribution/policies.json` nella cartella di installazione con backup/restore
- Marker `"_siteblocker":true` nel JSON per identificare i file creati da noi
- are_policies_active() controlla Chrome come campione + presenza del nostro JSON Firefox

### Espansione domini
- Input utente normalizzato: strip https://, www., m., path → root domain
- Espansione automatica: root → root + www.root + m.root
- Validazione: rifiutato se il dominio non contiene un punto (es. "netflix" senza TLD)
- Rimozione: usa HashSet per rimuovere root + tutte le varianti in un'operazione

### PIN
- argon2id, salt OsRng, hash self-describing (include params + salt nel JSON)
- Blocco NON richiede PIN. Apertura app e sblocco richiedono PIN.
- Minimo 4 caratteri (validato sia Rust che React)
- Reset PIN disponibile in Impostazioni (nessun PIN richiesto — chi usa l'app è già admin)
- Dopo reset: pin_hash = None → al prossimo avvio mostra schermata setup PIN

### i18n
- Context React in src/i18n.tsx con traduzioni `as const`
- `type Translations = (typeof translations)[Lang]` per evitare errori di tipo con union
- Lingua persistita in localStorage. Default: "it"
- Toggle ITA/ENG in header (pill style)

### Build e distribuzione
- Installer NSIS: `scripts/build.bat` → `target/release/bundle/nsis/`
- Portable exe: `scripts/build_portable.bat` → `YouTubeBlocker_vX.X.X.exe` nella root
- Bump versione: `scripts/bump_version.bat` → aggiorna tauri.conf.json + Cargo.toml + package.json
- Publisher: zoott28354 | License: MIT | Copyright © 2025 zoott28354

### Config
- `pin_hash: Option<String>` — None = primo avvio, mostra setup PIN
- `block_doh: bool` — default true

## Comandi Tauri disponibili
```
get_status()             → { hosts_blocked, firewall_active, browser_policy }
get_sites()              → Vec<String>
add_site(domain)         → Result<()>   // espande automaticamente www/m varianti
remove_site(domain)      → Result<()>   // rimuove root + varianti
block_all()              → Result<()>   // no PIN
unblock_all(pin)         → Result<()>   // PIN obbligatorio
has_pin()                → bool
set_pin(pin)             → Result<()>
change_pin(old, new)     → Result<()>
reset_pin()              → Result<()>   // azzera pin_hash → None
check_pin(pin)           → Result<()>   // solo verifica, nessun side effect (session lock)
```

## Preferenze utente
- NON includere "Co-Authored-By: Claude Sonnet 4.6" nei commit

## Roadmap / Feature future
- **Game Timer**: integrazione con un timer di gioco che blocca/sblocca automaticamente
- **Schedule**: blocco automatico per fascia oraria (es. 15:00-18:00 studio)
- **System tray**: toggle rapido senza aprire la finestra principale
- **Profili**: set di regole nominati (Studio, Lavoro, Weekend)
- **Mac/Linux**: hosts_path() già cross-platform; manca firewall (pfctl / iptables) e admin elevation
- **Toggle block_doh**: UI per abilitare/disabilitare firewall+policy separatamente dagli hosts

## Gotcha e bug noti risolti
- `tauri_build::Builder` non esiste in v2 → usare `tauri_build::build()`
- `requestedExecutionLevel` in tauri.conf.json non supportato da tauri-build 2.5.x
- `winres` in build.rs conflicta con tauri-build → rimosso
- ICO icon richiesta per build su Windows → generata con System.Drawing
- Hosts file svuotato su unblock → bug risolto con HashSet di righe esatte (non match parziale)
- `type Translations = typeof translations.it` causa errore TS con union → usare `(typeof translations)[Lang]`
- PowerShell script con $vars in `-Command` vengono strippati → scrivere script su file .ps1 e usare `-File`

## Come fare la build
```
scripts\setup.bat           # prima volta: installa npm packages
scripts\build.bat           # installer NSIS
scripts\build_portable.bat  # portable exe
```
