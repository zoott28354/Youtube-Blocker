# 🛡️ YouTube Blocker
### *...e altri siti che vuoi bloccare*

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows%2011-0078D4?logo=windows&logoColor=white)](https://github.com/zoott28354/YouTube-Blocker/releases)
[![Release](https://img.shields.io/github/v/release/zoott28354/YouTube-Blocker)](https://github.com/zoott28354/YouTube-Blocker/releases/latest)

App desktop per bloccare siti a livello di sistema su Windows.
Ideale per genitori che vogliono limitare l'accesso a YouTube e altri siti sui PC dei figli.

Agisce su tre livelli: **file hosts** + **regole firewall anti-DoH** + **Group Policy browser** —
nessun bypass possibile tramite DNS-over-HTTPS.

Richiede privilegi di amministratore. Protetto da PIN (argon2id).

---

## ✨ Caratteristiche

- 🔒 **Blocco a tre livelli** — hosts, firewall, policy browser
- 🔑 **PIN genitore** — hash argon2id, minimo 4 caratteri
- 🌐 **Più siti** — aggiungi qualsiasi dominio (varianti `www.` e `m.` automatiche)
- 💾 **Blocco persistente** — resta attivo dopo chiusura app e riavvio PC
- 🧹 **Ripristino al disinstall** — il wizard chiede se ripristinare tutto prima di rimuovere l'app
- 🌍 **Italiano / Inglese** — toggle in header
- 📦 **Installer perMachine** — installa per tutti gli utenti Windows

---

## ⚙️ Come funziona

| Operazione | Cosa succede |
|---|---|
| **Apertura app** | Richiede PIN — il figlio non può accedere senza |
| **Blocca** | Aggiunge voci `127.0.0.1` nel file hosts + regole firewall outbound verso DoH + Group Policy DoH nei browser |
| **Sblocca** | Richiede PIN → rimuove voci hosts + regole firewall + ripristina policy browser → flush DNS |

Il blocco **persiste dopo la chiusura dell'app** e dopo il riavvio del PC.
L'app non deve restare in esecuzione.

Lo sblocco è **anch'esso permanente**: una volta inserito il PIN e confermato, i siti tornano
accessibili anche dopo la chiusura dell'app e il riavvio del PC.
Per bloccare nuovamente è sufficiente riaprire l'app e premere **Blocca**.

---

## 🛡️ Livelli di blocco

| Livello | Cosa blocca |
|---|---|
| **🗂️ Hosts** | Reindirizza la risoluzione DNS a `127.0.0.1` per tutti i browser |
| **🔥 Firewall DoH** | Blocca il traffico TCP outbound verso IP DoH noti (Cloudflare, Google, Quad9) su porte 443 e 853 |
| **🌐 Policy browser** | Disabilita il DoH interno di Chrome, Edge, Brave e Firefox via Group Policy |

> I tre livelli insieme impediscono l'aggiramento tramite DNS-over-HTTPS,
> sia a livello di sistema operativo che di singolo browser.

---

## 🚀 Download

**[→ Scarica l'ultima versione](https://github.com/zoott28354/YouTube-Blocker/releases/latest)**

| File | Descrizione |
|---|---|
| `YouTubeBlocker_vX.X.X-setup.exe` | Installer NSIS (raccomandato) — installa per tutti gli utenti |
| `YouTubeBlocker_vX.X.X.exe` | Portable — nessun installer, esegui e basta |

---

## 🛠️ Stack

- **Tauri 2.x** — framework desktop
- **Rust** — backend (hosts, firewall, browser policy, auth, config)
- **React + TypeScript + Tailwind CSS** — frontend
- **i18n** — interfaccia Italiano / Inglese (toggle in header)

---

## 🔧 Sviluppo

### Prerequisiti

- Windows 11 (x64)
- [Rust](https://rustup.rs/) (`rustup` + `cargo`)
- [Node.js](https://nodejs.org/) 20+
- Microsoft C++ Build Tools (installabili da Visual Studio Installer)

### Avvio rapido

```
1. Esegui setup\setup.bat     → installa dipendenze npm e genera lancia.bat nella root
2. Doppio click su lancia.bat → eleva admin + avvia Tauri dev server
```

### Scripts (cartella `setup/`)

| Script | Cosa fa |
|---|---|
| `setup.bat` | Controlla prerequisiti (Node.js, Rust), installa npm, genera `lancia.bat` nella root |
| `build.bat` | Produce l'installer NSIS in `youtube-blocker/target/release/bundle/nsis/` |
| `build_portable.bat` | Produce `YouTubeBlocker_vX.X.X.exe` nella root del repo |
| `bump_version.bat` | Aggiorna versione in `tauri.conf.json`, `Cargo.toml` e `package.json` |

> `lancia.bat` è generato da `setup.bat` nella root del repo — non è in git (`.gitignore`).

### Prima build (da zero)

```
1. Clona il repo
2. Esegui setup\setup.bat        → installa npm packages
3. Esegui setup\build.bat        → compila tutto (prima volta: 15–30 min)
   oppure setup\build_portable.bat
```

### Cartelle generate (non nel git)

| Cartella | Dim. | Come si rigenera |
|---|---|---|
| `youtube-blocker/node_modules/` | ~400 MB | `setup.bat` / `npm install` |
| `youtube-blocker/dist/` | piccola | automatica |
| `youtube-blocker/.vite/` | piccola | automatica |
| `youtube-blocker/target/` | **2–5 GB** | automatica (~20 min) |

> Eliminare `target/` è sicuro ma richiede ricompilazione completa.
> Eliminare solo `target/debug/` libera spazio senza toccare la build di release.

---

## 📁 Architettura

```
YouTube-Blocker/
├── setup/
│   ├── setup.bat
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
            ├── Settings.tsx      — cambio PIN, reset PIN
            └── About.tsx         — versione, licenza, link GitHub
```

---

## 💾 Configurazione persistita

`%PROGRAMDATA%\YouTubeBlocker\config.json`

```json
{
  "sites": ["youtube.com", "www.youtube.com", "m.youtube.com", "youtu.be", "www.youtu.be", "m.youtu.be"],
  "pin_hash": "<argon2id hash>",
  "block_doh": true
}
```

- I siti si aggiungono/rimuovono dalla tab **Siti** — le varianti `www.` e `m.` vengono aggiunte automaticamente.
- Il PIN si gestisce dalla tab **Impostazioni**.
- PIN dimenticato? → **Impostazioni → Reset PIN** → al prossimo avvio si reimposta.

---

## 🔍 Verifica manuale

```powershell
# Voci nel file hosts
Get-Content "$env:SystemRoot\System32\drivers\etc\hosts" | Select-String "YouTubeBlocker"

# DNS risolve a 127.0.0.1 dopo blocco
Resolve-DnsName youtube.com

# Regola firewall attiva
netsh advfirewall firewall show rule name="YouTubeBlocker_DoH_1_1_1_1_p443"

# Policy browser Chrome
reg query "HKLM\SOFTWARE\Policies\Google\Chrome" /v DnsOverHttpsMode

# Config salvata
Get-Content "$env:PROGRAMDATA\YouTubeBlocker\config.json"
```

---

## 🗺️ Roadmap

- [ ] Integrazione Game Timer
- [ ] Blocchi programmati per orario (studio, notte)
- [ ] System tray (toggle rapido senza aprire la finestra)
- [ ] Profili (Lavoro, Studio, Weekend)
- [ ] Toggle block_doh separato da hosts
- [ ] Supporto macOS (`pfctl` + `dscacheutil`)
- [ ] Supporto Linux (`iptables`/`ufw` + `systemd-resolve`)

---

## 📄 Licenza

MIT © 2025 zoott28354
