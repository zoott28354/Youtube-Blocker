# YouTube Blocker — Contesto per Claude / Codex Mac

## Cosa fa questo progetto
App desktop Tauri 2 per bloccare siti a livello di sistema.
Usecase principale: genitore blocca YouTube (e altri siti) per un figlio di 11 anni.
Su Windows il blocco usa tre livelli: file `hosts` + regole Windows Firewall anti-DoH + Group Policy browser.
PIN argon2id richiesto per aprire l'app e per sbloccare. Interfaccia bilingue IT/EN.

### Stato branch `multios`
- Windows: blocco completo a tre livelli (`hosts` + firewall anti-DoH + browser policy)
- macOS: blocco tramite `hosts` + browser policy silenziosa (disabilita DoH nei browser per evitare ritardi da DNS cache). La UI mostra solo BLOCCATO/SBLOCCATO senza dettagli sui layer.
- Linux e' secondario: stessa idea di macOS, ma non e' il focus immediato

## Stack
- Tauri 2.x + React + TypeScript + Tailwind CSS
- Rust backend in `youtube-blocker/src-tauri/src/`

## Struttura repo
```
YouTube-Blocker/
├── setup/              — bat file per setup/dev/build/bump_version
├── LICENSE               — MIT, Copyright 2025 zoott28354
├── README.md
├── CLAUDE.md
└── youtube-blocker/      — progetto Tauri
    ├── src-tauri/src/
    ├── src/
    └── public/
```

## Cartelle generate (NON nel git, eliminabili)
| Cartella | Come si rigenera |
|---|---|
| `youtube-blocker/node_modules/` | `setup.bat` / `npm install` |
| `youtube-blocker/dist/` | automatica durante build |
| `youtube-blocker/.vite/` | automatica |
| `youtube-blocker/target/` | automatica (lenta, ~20 min prima volta) |

## File chiave

| File | Responsabilità |
|---|---|
| `src-tauri/src/main.rs` | Comandi Tauri, admin check runtime, AppState (Mutex) |
| `src-tauri/src/config.rs` | AppConfig, load/save `%PROGRAMDATA%\YouTubeBlocker\config.json` (condiviso tra utenti) |
| `src-tauri/src/auth.rs` | hash_pin / verify_pin con argon2id |
| `src-tauri/src/hosts.rs` | Lettura/scrittura file hosts + flush DNS multipiattaforma |
| `src-tauri/src/firewall.rs` | Windows: regole netsh outbound TCP verso DoH su porte 443 e 853 (non usato su macOS) |
| `src-tauri/src/browsers.rs` | Disabilita DoH nei browser. Windows: registry + policies.json. macOS: managed prefs plist + policies.json (silente, non mostrato in UI) |
| `src/i18n.tsx` | Traduzioni IT/EN, LangProvider context, useI18n hook |
| `src/hooks/useBlocker.ts` | Stato React centralizzato, tutti gli invoke Tauri |
| `src/App.tsx` | Routing tab, session lock, setup PIN, toggle lingua |

## Decisioni architetturali importanti

### Admin elevation
Non usiamo manifest UAC (non supportato da tauri-build 2.5.x in tauri.conf.json).
Admin check a runtime: `net session` su Windows. Se non admin → rilancio con
`Start-Process -Verb RunAs` via PowerShell nascosto (CREATE_NO_WINDOW).
Nel branch `multios` c'e' anche un primo flusso macOS con `osascript` per rilancio con privilegi admin.

### Session lock (PIN all'apertura)
- All'apertura dell'app, se PIN è impostato, viene mostrata una schermata di verifica PIN
- Solo dopo autenticazione si accede all'interfaccia principale
- `isSessionUnlocked` in useBlocker.ts — false al mount, true dopo check_pin con successo
- Sblocco siti richiede ancora PIN (difesa in profondità)
- Reset PIN azzera anche la sessione corrente

### Hosts file
- Marker: `# YouTubeBlocker <domain>` identifica le righe nostre
- Rimozione: match ESATTO sulle righe scritte da noi (HashSet), NON match parziale
  su "127.0.0.1" + dominio (bug storico che svuotava il file hosts)
- Line endings: `\r\n` su Windows, `\n` su Mac/Linux (cfg! compile-time)
- flush_dns() chiamato dopo ogni write

### Firewall (solo Windows)
- Nome regole: `YouTubeBlocker_DoH_<ip>_p<porta>` (deterministico)
- Remove-before-add: idempotente, sicuro chiamare più volte
- Solo TCP (DoH su QUIC/UDP porta 443 non bloccato — possibile v2)
- `is_firewall_supported()` ritorna true solo su Windows
- Su macOS il firewall pfctl e' stato rimosso: le regole non persistevano al reboot (pf disabilitato di default)

### Browser DoH (browsers.rs)
- **Windows**: Chrome/Edge/Brave/Vivaldi/Opera/Chromium via registry `HKLM\SOFTWARE\Policies\...\DnsOverHttpsMode = "off"`. Firefox via `distribution/policies.json`
- **macOS**: Chromium browsers via plist in `/Library/Managed Preferences/`. Firefox via `distribution/policies.json`. Applicata silenziosamente al blocco/sblocco per forzare bypass immediato della DNS cache dei browser. Non persistono al reboot ma non serve: hosts persiste e la cache DNS e' vuota dopo il riavvio.
- `is_browser_policy_supported()` ritorna true solo su Windows (la UI non mostra il layer policy su macOS)
- Marker `"_youtubeblocker":true` nel JSON Firefox per identificare i file creati da noi

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
- Installer NSIS: `setup/build.bat` → `youtube-blocker/target/release/bundle/nsis/`
- Portable exe: `setup/build_portable.bat` → `YouTubeBlocker_vX.X.X.exe` nella root
- Bump versione: `setup/bump_version.bat` → aggiorna tauri.conf.json + Cargo.toml + package.json
- Publisher: zoott28354 | License: MIT | Copyright © 2025 zoott28354
- **installMode: "perMachine"** in tauri.conf.json → installa in `C:\Program Files` per tutti gli utenti
- **CARGO_TARGET_DIR** impostato nei bat (`set CARGO_TARGET_DIR=%CD%\target`) → Cargo scrive in
  `youtube-blocker/target/` invece di `youtube-blocker/src-tauri/target/` (più ordinato, stesso path da gitignore)
- Dev: `setup/setup.bat` genera `lancia.bat` nella root del repo dopo npm install (non in git)
  `lancia.bat` eleva admin + avvia `npm run tauri dev` → produce `target/debug/youtube-blocker.exe`

### Config
- `pin_hash: Option<String>` — None = primo avvio, mostra setup PIN
- `block_doh: bool` — default true
- Percorso Windows: `%PROGRAMDATA%\YouTubeBlocker\config.json` (es. `C:\ProgramData\YouTubeBlocker\`)
  - **Perché PROGRAMDATA e non LOCALAPPDATA**: PROGRAMDATA è condiviso tra tutti gli account Windows
    e scrivibile solo con privilegi admin. Essenziale per installazioni perMachine: se il config fosse
    in LOCALAPPDATA l'utente figlio avrebbe un config separato senza PIN impostato.
  - Su Mac/Linux fallback a `dirs::data_local_dir()`

### Stato protezione per OS
- `get_status()` espone `os_name`, `firewall_supported`, `browser_policy_supported`
- La UI determina BLOCCATO/SBLOCCATO basandosi solo su `hosts_blocked`
- Su Windows la UI mostra anche gli indicatori per firewall e browser policy
- Su macOS la UI mostra solo BLOCCATO/SBLOCCATO senza dettagli layer
- Startup: se hosts risulta bloccato, le liste restano attive; se hosts non e' bloccato, le liste vengono azzerate

## Comandi Tauri disponibili
```
get_status()             → { hosts_blocked, firewall_active, browser_policy, os_name, firewall_supported, browser_policy_supported }
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
- **Mac/Linux**: macOS funzionante con hosts + browser policy silenziosa. Linux da implementare.
- **Toggle block_doh**: UI per abilitare/disabilitare firewall+policy separatamente dagli hosts (Windows)

## Gotcha e bug noti risolti
- `tauri_build::Builder` non esiste in v2 → usare `tauri_build::build()`
- `requestedExecutionLevel` in tauri.conf.json non supportato da tauri-build 2.5.x
- `winres` in build.rs conflicta con tauri-build → rimosso
- ICO icon richiesta per build su Windows → generata con System.Drawing
- Hosts file svuotato su unblock → bug risolto con HashSet di righe esatte (non match parziale)
- `type Translations = typeof translations.it` causa errore TS con union → usare `(typeof translations)[Lang]`
- PowerShell script con $vars in `-Command` vengono strippati → scrivere script su file .ps1 e usare `-File`
- `variant NoAppData is never constructed` (warning Rust su Windows) → aggiungere `#[cfg(not(target_os = "windows"))]`
  sia alla variante dell'enum che al ramo di codice che la costruisce
- Batch: `(s/n):` dentro blocchi `if (...)` causa `: non atteso.` — la `)` chiude prematuramente il blocco.
  Usare `[s/n]` oppure riscrivere con `goto` (nessun blocco parentesizzato per control flow)
- `setup.bat` mostra i prerequisiti (Node.js, Rust) con [OK]/[MANCANTE] già nel menu, prima che l'utente
  selezioni un'opzione, usando `where node` / `where cargo` (più affidabile di eseguire il tool direttamente)

## Come fare la build
```
setup\setup.bat           # prima volta: npm install + genera lancia.bat nella root
lancia.bat                # avvia dev mode (generato da setup.bat, non in git)
setup\build.bat           # installer NSIS
setup\build_portable.bat  # portable exe
```
