# YouTube Blocker
### *...and other sites you want to block*

App desktop per bloccare siti a livello di sistema su Windows (Mac/Linux in roadmap).
Modifica il file `hosts` + regole Windows Firewall per bloccare i resolver DNS-over-HTTPS,
e imposta Group Policy nei browser per disabilitare il DoH interno.

Richiede privilegi di amministratore. Protetto da PIN genitore (argon2id).

---

## Funzionamento

| Operazione | Cosa succede |
|---|---|
| **Apertura app** | Richiede PIN — il figlio non può accedere senza |
| **Blocca** | Aggiunge voci `127.0.0.1` nel file hosts + regole firewall outbound verso DoH + Group Policy DoH nei browser |
| **Sblocca** | Richiede PIN → rimuove hosts + firewall + ripristina policy browser → flush DNS |

Il blocco **persiste dopo la chiusura dell'app** e dopo il riavvio del PC.
L'app non deve restare in esecuzione.

---

## Livelli di blocco

| Livello | Cosa blocca |
|---|---|
| **Hosts** | Risoluzione DNS del dominio → `127.0.0.1` per tutti i browser |
| **Firewall DoH** | Traffico TCP outbound verso IP DoH noti (Cloudflare, Google, Quad9) su porte 443 e 853 |
| **Policy browser** | Disabilita il DoH interno di Chrome, Edge, Brave e Firefox via Group Policy |

---

## Stack

- **Tauri 2.x** — framework desktop
- **Rust** — backend (hosts, firewall, browser policy, auth, config)
- **React + TypeScript + Tailwind CSS** — frontend
- **i18n** — interfaccia in Italiano e Inglese (toggle in header)

---

## Prerequisiti

- Windows 11 (x64)
- [Rust](https://rustup.rs/) (`rustup` + `cargo`)
- [Node.js](https://nodejs.org/) 20+
- Microsoft C++ Build Tools (installabili da Visual Studio Installer)

---

## Scripts (cartella `setup/`)

Tutti gli script vanno eseguiti con doppio click dalla cartella `setup/`.

| Script | Cosa fa |
|---|---|
| `setup.bat` | Installa le dipendenze npm — eseguire dopo il primo clone |
| `dev.bat` | Avvia in modalità sviluppo — eleva i privilegi admin automaticamente |
| `build.bat` | Produce l'installer NSIS in `youtube-blocker/src-tauri/target/release/bundle/nsis/` |
| `build_portable.bat` | Produce `YouTubeBlocker_vX.X.X.exe` nella root del repo (senza installer) |
| `bump_version.bat` | Aggiorna la versione in `tauri.conf.json`, `Cargo.toml` e `package.json` in un colpo |

> In dev mode serve essere **Amministratore**. `dev.bat` gestisce l'elevazione da solo.
> In produzione l'UAC prompt appare automaticamente al lancio dell'app.

---

## Prima build (da zero)

```
1. Clona il repo
2. Esegui setup\setup.bat        → installa npm packages
3. Esegui setup\build.bat        → compila tutto (prima volta: 15-30 min)
   oppure setup\build_portable.bat
```

### Cartelle generate dalla build (non nel git, eliminabili per liberare spazio)

| Cartella | Dimensione | Come si rigenera |
|---|---|---|
| `youtube-blocker/node_modules/` | ~400 MB | `setup.bat` o `npm install` |
| `youtube-blocker/dist/` | piccola | automatica durante la build |
| `youtube-blocker/.vite/` | piccola | automatica |
| `youtube-blocker/target/` | **2–5 GB** | automatica (lenta, ~20 min) |

> Eliminare `target/` è sicuro ma richiede una ricompilazione completa.
> Eliminare solo `target/debug/` libera spazio senza toccare la build di release.

---

## Configurazione persistita

`%PROGRAMDATA%\YouTubeBlocker\config.json`

```json
{
  "sites": ["youtube.com", "www.youtube.com", "m.youtube.com", "youtu.be", "www.youtu.be", "m.youtu.be"],
  "pin_hash": "<argon2id hash>",
  "block_doh": true
}
```

- I siti si aggiungono/rimuovono dalla tab **Siti**. Inserire il dominio root (es. `netflix.com`) — le varianti `www.` e `m.` vengono aggiunte automaticamente.
- Il PIN si cambia dalla tab **Impostazioni**.
- Se il PIN viene dimenticato: tab **Impostazioni** → **Reset PIN** → al prossimo avvio verrà richiesto di impostarne uno nuovo.

---

## Architettura

```
YouTube-Blocker/
├── setup/
│   ├── setup.bat
│   ├── dev.bat
│   ├── build.bat
│   ├── build_portable.bat
│   └── bump_version.bat
└── youtube-blocker/
    ├── src-tauri/src/
    │   ├── main.rs       — comandi Tauri, admin check, AppState
    │   ├── config.rs     — AppConfig, load/save JSON in %PROGRAMDATA%\YouTubeBlocker\
    │   ├── auth.rs       — PIN con argon2id
    │   ├── hosts.rs      — blocco/sblocco file hosts + flush DNS
    │   ├── firewall.rs   — regole netsh per DNS-over-HTTPS
    │   └── browsers.rs   — Group Policy DoH per Chrome/Edge/Brave/Firefox
    └── src/
        ├── i18n.tsx              — traduzioni IT/EN, LangProvider, useI18n
        ├── App.tsx               — routing tab, session lock, setup PIN
        ├── hooks/useBlocker.ts   — stato centralizzato, invoke Tauri
        └── components/
            ├── StatusCard.tsx    — badge stato, 3 indicatori, icona
            ├── SiteList.tsx      — lista siti raggruppata, aggiunta/rimozione
            ├── PinModal.tsx      — modal setup e verifica PIN
            └── Settings.tsx      — cambio PIN, reset PIN
```

---

## Comandi Tauri disponibili

| Comando | Descrizione |
|---|---|
| `get_status()` | `{ hosts_blocked, firewall_active, browser_policy }` |
| `get_sites()` | Lista domini bloccati |
| `add_site(domain)` | Aggiunge dominio + varianti www/m |
| `remove_site(domain)` | Rimuove dominio + varianti |
| `block_all()` | Blocca tutto (no PIN) |
| `unblock_all(pin)` | Sblocca tutto (PIN obbligatorio) |
| `has_pin()` | `true` se PIN impostato |
| `set_pin(pin)` | Imposta PIN (primo avvio) |
| `change_pin(old, new)` | Cambia PIN (richiede PIN attuale) |
| `reset_pin()` | Azzera PIN → riparte da setup al prossimo avvio |
| `check_pin(pin)` | Verifica PIN senza side effects (usato per session lock) |

---

## Roadmap

- [ ] Integrazione Game Timer
- [ ] Blocchi programmati per orario (studio, notte)
- [ ] Supporto macOS (`pfctl` + `dscacheutil`)
- [ ] Supporto Linux (`iptables`/`ufw` + `systemd-resolve`)
- [ ] System tray (blocco rapido senza aprire la finestra)
- [ ] Profili (Lavoro, Studio, Weekend)
- [ ] Toggle block_doh separato da hosts

---

## Verifica manuale

```powershell
# Dopo blocco: voci nel file hosts
Get-Content "$env:SystemRoot\System32\drivers\etc\hosts" | Select-String "YouTubeBlocker"

# Dopo blocco: DNS risolve a 127.0.0.1
Resolve-DnsName youtube.com

# Regole firewall attive
netsh advfirewall firewall show rule name="YouTubeBlocker_DoH_1_1_1_1_p443"

# Policy browser Chrome
reg query "HKLM\SOFTWARE\Policies\Google\Chrome" /v DnsOverHttpsMode

# Configurazione salvata
Get-Content "$env:PROGRAMDATA\YouTubeBlocker\config.json"
```
