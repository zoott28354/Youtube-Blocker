# YouTube Blocker
### *...and other sites you want to block*

App desktop per bloccare siti a livello di sistema su Windows (Mac/Linux in roadmap).
Modifica il file `hosts` + regole Windows Firewall per bloccare i resolver DNS-over-HTTPS, rendendo il blocco resistente ai browser moderni (Chrome, Firefox, Edge).

Richiede privilegi di amministratore. Protetto da PIN genitore.

---

## Funzionamento

| Operazione | Cosa succede |
|---|---|
| **Blocca** | Aggiunge voci `127.0.0.1` nel file hosts + regole firewall outbound verso DoH (Cloudflare, Google, Quad9) |
| **Sblocca** | Richiede PIN → rimuove voci hosts + regole firewall → flush DNS |

Il blocco **persiste dopo la chiusura dell'app** e dopo il riavvio del PC.
L'app non deve restare in esecuzione.

---

## Stack

- **Tauri 2.x** — framework desktop
- **Rust** — backend (hosts, firewall, auth, config)
- **React + TypeScript + Tailwind CSS** — frontend

---

## Prerequisiti

- Windows 11 (x64)
- [Rust](https://rustup.rs/) (`rustup` + `cargo`)
- [Node.js](https://nodejs.org/) 20+
- Microsoft C++ Build Tools (installabili da Visual Studio Installer)

---

## Build & Run

```bash
cd site-blocker
npm install

# Sviluppo (terminale come Amministratore)
npm run tauri dev

# Build distribuzione
npm run tauri build
# Output: src-tauri/target/release/bundle/nsis/SiteBlocker_x64-setup.exe
```

> In dev mode il terminale deve girare come **Amministratore**.
> In produzione l'UAC prompt appare automaticamente al lancio.

---

## Configurazione

La configurazione è salvata in `%LOCALAPPDATA%\SiteBlocker\config.json`:

```json
{
  "sites": ["youtube.com", "www.youtube.com", "m.youtube.com", "youtu.be"],
  "pin_hash": "<argon2id hash>",
  "block_doh": true
}
```

I siti si aggiungono/rimuovono dalla tab **Siti** nell'app.
Il PIN si cambia dalla tab **Impostazioni**.

---

## Architettura

```
site-blocker/
├── src-tauri/src/
│   ├── main.rs       — comandi Tauri, admin check, AppState
│   ├── config.rs     — AppConfig, load/save JSON
│   ├── auth.rs       — PIN con argon2id
│   ├── hosts.rs      — blocco/sblocco file hosts + flush DNS
│   └── firewall.rs   — regole netsh per DNS-over-HTTPS
└── src/
    ├── App.tsx               — routing, primo avvio
    ├── hooks/useBlocker.ts   — stato centralizzato
    └── components/
        ├── StatusCard.tsx
        ├── SiteList.tsx
        ├── PinModal.tsx
        └── Settings.tsx
```

---

## Roadmap

- [ ] Integrazione Game Timer
- [ ] Blocchi programmati per orario (studio, notte)
- [ ] Supporto macOS (`pfctl` + `dscacheutil`)
- [ ] Supporto Linux (`iptables`/`ufw` + `systemd-resolve`)
- [ ] System tray (blocco rapido senza aprire la finestra)
- [ ] Profili (Lavoro, Studio, Weekend)
- [ ] CHANGELOG

---

## Verifica manuale

```powershell
# Dopo blocco: voci nel file hosts
Get-Content "$env:SystemRoot\System32\drivers\etc\hosts" | Select-String "SiteBlocker"

# Dopo blocco: DNS risolve a 127.0.0.1
Resolve-DnsName youtube.com

# Regole firewall attive
netsh advfirewall firewall show rule name="SiteBlocker_DoH_1_1_1_1_p443"

# Configurazione salvata
Get-Content "$env:LOCALAPPDATA\SiteBlocker\config.json"
```
