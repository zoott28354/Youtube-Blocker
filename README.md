# 🛡️ YouTube Blocker
### *...e altri siti che vuoi bloccare*

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform: Windows + macOS](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS-0078D4?logo=windows&logoColor=white)](https://github.com/zoott28354/YouTube-Blocker/releases)
[![Release](https://img.shields.io/github/v/release/zoott28354/YouTube-Blocker)](https://github.com/zoott28354/YouTube-Blocker/releases/latest)

App desktop per bloccare siti a livello di sistema su **Windows** e **macOS**.
Ideale per genitori che vogliono limitare l'accesso a YouTube e altri siti sui PC dei figli.

Lo lanci, inserisci il PIN e selezioni la lista preimpostata da bloccare (video, games, ecc.) poi **BLOCCA**!
Resterà bloccato fino a quando non riaprirai l'app e premi **SBLOCCA**.
Puoi anche duplicare una lista e modificarla oppure crearne una nuova (per i siti per adulti per esempio).

Agisce su tre livelli: **file hosts** + **regole firewall anti-DoH** + **policy browser** — nessun bypass possibile tramite DNS-over-HTTPS.

Richiede privilegi di amministratore. Protetto da PIN (argon2id).

---

## ✨ Caratteristiche

- 🔒 **Blocco a tre livelli** — hosts, firewall DoH, policy browser
- 🔑 **PIN genitore** — hash argon2id, minimo 4 caratteri
- 📚 **Liste di blocco** — preset per video, browser games, gaming platforms, chat, social e streaming + liste personalizzate
- 🌐 **31 varianti per dominio** — blocca automaticamente www, m e 29 sottodomini locale (it, en, fr, de, ecc.)
- 💾 **Blocco persistente** — resta attivo dopo chiusura app e riavvio PC/Mac
- 🌍 **Italiano / Inglese** — toggle in header
- 🖥️ **Windows + macOS** — blocco completo a tre livelli su entrambi

---

## ⚙️ Come funziona

| Operazione | Cosa succede |
|---|---|
| **Apertura app** | Richiede password admin (macOS) o privilegi admin (Windows), poi PIN |
| **Blocca** | Applica le liste attive: voci `127.0.0.1` e `::1` nel file hosts + regole firewall DoH + policy browser |
| **Sblocca** | Richiede PIN → rimuove voci hosts + regole firewall + ripristina policy browser → flush DNS |

Il blocco **persiste dopo la chiusura dell'app** e dopo il riavvio del PC/Mac.
L'app non deve restare in esecuzione.

---

## 🛡️ Livelli di blocco

| Livello | Windows | macOS |
|---|---|---|
| **🗂️ Hosts** | `127.0.0.1` + `::1` in `C:\Windows\System32\drivers\etc\hosts` | `127.0.0.1` + `::1` in `/etc/hosts` |
| **🔥 Firewall DoH** | Regole `netsh` outbound TCP verso DoH noti (porta 443/853) | Regole `pfctl` (packet filter) verso DoH noti (porta 443/853) |
| **🌐 Policy browser** | Registry `HKLM\SOFTWARE\Policies\...` per Chromium + `policies.json` per Firefox | Plist in `/Library/Managed Preferences/` per Chromium + `policies.json` per Firefox |

> I tre livelli insieme impediscono l'aggiramento tramite DNS-over-HTTPS,
> sia a livello di sistema operativo che di singolo browser.

### Browser con policy DoH gestita

| Browser | Windows | macOS |
|---|---|---|
| Google Chrome | Registry | Managed Preferences plist |
| Microsoft Edge | Registry | Managed Preferences plist |
| Brave | Registry | Managed Preferences plist |
| Vivaldi | Registry | Managed Preferences plist |
| Opera / Opera GX | Registry | Managed Preferences plist |
| Chromium | Registry | Managed Preferences plist |
| Firefox | `policies.json` | `policies.json` |

> Se il browser non è installato, l'operazione viene ignorata senza errori.
> Hosts e firewall bloccano comunque il DNS per **qualsiasi** browser.

---

## 🚀 Download

**[→ Scarica l'ultima versione](https://github.com/zoott28354/YouTube-Blocker/releases/latest)**

### Windows

| File | Descrizione |
|---|---|
| `YouTubeBlocker_X.X.X_x64-setup.exe` | Installer NSIS — installa per tutti gli utenti |

> Al doppio click sul setup, Windows potrebbe mostrare un avviso SmartScreen ("App non riconosciuta").
> Clicca **Ulteriori informazioni → Esegui comunque**.

### macOS

Installa tramite [Homebrew](https://brew.sh/):

```
brew tap zoott28354/youtube-blocker
brew install --cask youtube-blocker
```

Al primo avvio l'app chiederà la password di amministratore.

---

## 🛠️ Stack

- **Tauri 2.x** — framework desktop
- **Rust** — backend (hosts, firewall, browser policy, auth, config)
- **React + TypeScript + Tailwind CSS** — frontend
- **i18n** — interfaccia Italiano / Inglese (toggle in header)
- **GitHub Actions** — build automatico Windows + macOS

---

## 🔧 Sviluppo

### Prerequisiti

- Windows 11 (x64) o macOS 10.15+ (Apple Silicon / Intel)
- [Rust](https://rustup.rs/) (`rustup` + `cargo`)
- [Node.js](https://nodejs.org/) 20+
- Windows: Microsoft C++ Build Tools (Visual Studio Installer)
- macOS: Xcode Command Line Tools (`xcode-select --install`)

### Avvio rapido (Windows)

```
1. Esegui setup\setup.bat     → installa dipendenze npm e genera lancia.bat nella root
2. Doppio click su lancia.bat → eleva admin + avvia Tauri dev server
```

### Avvio rapido (macOS)

```
1. cd youtube-blocker && npm install
2. sudo npm run tauri dev
```

### Build

| Piattaforma | Comando | Output |
|---|---|---|
| Windows | `setup\build.bat` | `youtube-blocker/target/release/bundle/nsis/*.exe` |
| macOS | `cd youtube-blocker && npm run tauri build` | `src-tauri/target/release/bundle/dmg/*.dmg` |

> La CI su GitHub Actions builda automaticamente entrambe le piattaforme ad ogni tag `v*`.

---

## 📁 Architettura

```
YouTube-Blocker/
├── .github/workflows/
│   └── build.yml         — CI: build Windows + macOS, crea release
├── homebrew/
│   └── youtube-blocker.rb — formula Homebrew Cask per macOS
├── setup/
│   ├── setup.bat
│   ├── build.bat
│   └── bump_version.bat
└── youtube-blocker/
    ├── src-tauri/src/
    │   ├── main.rs       — comandi Tauri, admin check (Windows + macOS), AppState
    │   ├── config.rs     — AppConfig, liste predefinite, sync, migration
    │   ├── auth.rs       — PIN con argon2id
    │   ├── hosts.rs      — blocco/sblocco file hosts + flush DNS (cross-platform)
    │   ├── firewall.rs   — Windows: netsh | macOS: pfctl
    │   └── browsers.rs   — Windows: registry | macOS: managed preferences plist
    └── src/
        ├── i18n.tsx              — traduzioni IT/EN
        ├── App.tsx               — routing tab, session lock, setup PIN
        ├── hooks/useBlocker.ts   — stato centralizzato, invoke Tauri
        └── components/
            ├── StatusCard.tsx    — badge stato, indicatori per livello
            ├── BlockLists.tsx    — preset e liste personalizzate, toggle, edit, duplica
            ├── PinModal.tsx      — modal setup e verifica PIN
            ├── Settings.tsx      — cambio PIN, reset PIN
            └── About.tsx         — versione, licenza, link GitHub
```

---

## 💾 Configurazione persistita

- **Windows**: `%PROGRAMDATA%\YouTubeBlocker\config.json`
- **macOS**: `/Library/Application Support/YouTubeBlocker/config.json`

```json
{
  "lists": [
    {
      "id": "builtin-youtube",
      "name": "YouTube & Video",
      "sites": ["youtube.com", "www.youtube.com", "m.youtube.com", "..."],
      "active": false,
      "builtin": true
    }
  ],
  "pin_hash": "<argon2id hash>",
  "block_doh": true
}
```

- Le liste si gestiscono dalla tab **Liste**.
- Le preset incluse coprono video, browser games, gaming platforms, chat/messaging, social e streaming.
- Puoi duplicare una lista predefinita per farne una personalizzata.
- Se l'app apre in stato **Sbloccato**, le liste attive residue vengono spente automaticamente.
- PIN dimenticato? → **Impostazioni → Reset PIN** → al prossimo avvio si reimposta.

---

## 📄 Licenza

MIT © 2025 zoott28354
