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
- 📚 **Liste di blocco** — preset per YouTube, giochi, social e streaming + liste personalizzate
- 💾 **Blocco persistente** — resta attivo dopo chiusura app e riavvio PC
- 🧹 **Ripristino al disinstall** — il wizard chiede se ripristinare tutto prima di rimuovere l'app
- 🌍 **Italiano / Inglese** — toggle in header
- 📦 **Installer perMachine** — installa per tutti gli utenti Windows

---

## ⚙️ Come funziona

| Operazione | Cosa succede |
|---|---|
| **Apertura app** | Richiede PIN — il figlio non può accedere senza |
| **Blocca** | Applica le liste attive: voci `127.0.0.1` nel file hosts + regole firewall outbound verso DoH + Group Policy DoH nei browser |
| **Sblocca** | Richiede PIN → rimuove voci hosts + regole firewall + ripristina policy browser → flush DNS e spegne tutte le liste attive |

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
| **🌐 Policy browser** | Disabilita il DoH interno dei principali browser via Group Policy |

> I tre livelli insieme impediscono l'aggiramento tramite DNS-over-HTTPS,
> sia a livello di sistema operativo che di singolo browser.

### Browser con policy DoH gestita

| Browser | Metodo |
|---|---|
| Google Chrome | Registry `HKLM\SOFTWARE\Policies\Google\Chrome` |
| Microsoft Edge | Registry `HKLM\SOFTWARE\Policies\Microsoft\Edge` |
| Brave | Registry `HKLM\SOFTWARE\Policies\BraveSoftware\Brave` |
| Vivaldi | Registry `HKLM\SOFTWARE\Policies\Vivaldi` |
| Opera / Opera GX | Registry `HKLM\SOFTWARE\Policies\Opera Software\Opera` |
| Chromium | Registry `HKLM\SOFTWARE\Policies\Chromium` |
| Firefox | `distribution/policies.json` nella cartella di installazione |

> Se il browser non è installato, l'operazione viene ignorata senza errori.
> Hosts e firewall bloccano comunque il DNS per **qualsiasi** browser.

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
    │   └── browsers.rs   — Group Policy DoH per Chrome/Edge/Brave/Vivaldi/Opera/Firefox
    └── src/
        ├── i18n.tsx              — traduzioni IT/EN, LangProvider, useI18n
        ├── App.tsx               — routing tab, session lock, setup PIN
        ├── hooks/useBlocker.ts   — stato centralizzato, invoke Tauri
        └── components/
            ├── StatusCard.tsx    — badge stato, 3 indicatori, liste attive, icona
            ├── BlockLists.tsx    — preset e liste personalizzate, toggle, edit, duplica
            ├── PinModal.tsx      — modal setup e verifica PIN
            ├── Settings.tsx      — cambio PIN, reset PIN
            └── About.tsx         — versione, licenza, link GitHub
```

---

## 💾 Configurazione persistita

`%PROGRAMDATA%\YouTubeBlocker\config.json`

```json
{
  "lists": [
    {
      "id": "builtin-youtube",
      "name": "YouTube",
      "sites": ["youtube.com", "www.youtube.com", "m.youtube.com"],
      "active": false,
      "builtin": true
    }
  ],
  "pin_hash": "<argon2id hash>",
  "block_doh": true
}
```

- Le liste si gestiscono dalla tab **Liste**.
- In modifica, le varianti `www.` e `m.` restano visibili e modificabili come voci separate.
- Se l'app apre in stato **Sbloccato**, le liste attive residue vengono riallineate e spente automaticamente.
- Il PIN si gestisce dalla tab **Impostazioni**.
- PIN dimenticato? → **Impostazioni → Reset PIN** → al prossimo avvio si reimposta.

---

## 📄 Licenza

MIT © 2025 zoott28354
