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

Richiede privilegi di amministratore. Protetto da PIN (argon2id).

---

## ✨ Caratteristiche

- 🔒 **Blocco persistente** — resta attivo dopo chiusura app e riavvio PC/Mac
- 🔑 **PIN genitore** — richiesto per sbloccare
- 📚 **Liste di blocco** — preset per video, browser games, gaming platforms, chat, social e streaming + liste personalizzate
- 🌐 **31 varianti per dominio** — blocca automaticamente www, m e 29 sottodomini locale (it, en, fr, de, ecc.)
- 🌍 **Italiano / Inglese** — toggle in header
- 🖥️ **Windows + macOS**

---

## ⚙️ Come funziona

| Operazione | Cosa succede |
|---|---|
| **Apertura app** | Richiede password admin (macOS) o privilegi admin (Windows), poi PIN |
| **Blocca** | Applica le liste attive — il blocco è immediato |
| **Sblocca** | Richiede PIN → rimuove il blocco |

Il blocco **persiste dopo la chiusura dell'app** e dopo il riavvio del PC/Mac.
L'app non deve restare in esecuzione.

---

## 🚀 Download

**[→ Scarica l'ultima versione](https://github.com/zoott28354/YouTube-Blocker/releases/latest)**

### Windows

| File | Descrizione |
|---|---|
| `YouTubeBlocker_X.X.X_x64-setup.exe` | Installer — installa per tutti gli utenti |

> Al doppio click sul setup, Windows potrebbe mostrare un avviso SmartScreen ("App non riconosciuta").
> Clicca **Ulteriori informazioni → Esegui comunque**.

### macOS

> **L'app non è firmata con certificato Apple Developer**, quindi macOS blocca l'apertura del .dmg scaricato direttamente. L'unico metodo di installazione supportato è tramite [Homebrew](https://brew.sh/).

```
brew tap zoott28354/youtube-blocker
brew install --cask youtube-blocker
```

Al primo avvio l'app chiederà la password di amministratore.

---

## 💾 Note

- Le liste si gestiscono dalla tab **Liste**.
- Le preset incluse coprono video, browser games, gaming platforms, chat/messaging, social e streaming.
- Puoi duplicare una lista predefinita per farne una personalizzata.
- PIN dimenticato? → **Impostazioni → Reset PIN** → al prossimo avvio si reimposta.

---

## 📄 Licenza

MIT © 2025 zoott28354
