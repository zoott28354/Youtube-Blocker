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
| **Blocca** | Aggiunge voci `127.0.0.1` nel file hosts + regole firewall outbound verso DoH (Cloudflare, Google, Quad9) + Group Policy DoH nei browser |
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
site-blocker/
├── src-tauri/src/
│   ├── main.rs       — comandi Tauri, admin check, AppState
│   ├── config.rs     — AppConfig, load/save JSON in %LOCALAPPDATA%\SiteBlocker\
│   ├── auth.rs       — PIN con argon2id
│   ├── hosts.rs      — blocco/sblocco file hosts + flush DNS
│   ├── firewall.rs   — regole netsh per DNS-over-HTTPS
│   └── browsers.rs   — Group Policy DoH per Chrome/Edge/Brave/Firefox
└── src/
    ├── i18n.tsx              — traduzioni IT/EN, LangProvider, useI18n
    ├── App.tsx               — routing tab, primo avvio setup PIN
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
| `change_pin(old, new)` | Cambia PIN |
| `reset_pin()` | Azzera PIN (riparte da setup) |

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
Get-Content "$env:SystemRoot\System32\drivers\etc\hosts" | Select-String "SiteBlocker"

# Dopo blocco: DNS risolve a 127.0.0.1
Resolve-DnsName youtube.com

# Regole firewall attive
netsh advfirewall firewall show rule name="SiteBlocker_DoH_1_1_1_1_p443"

# Policy browser Chrome
reg query "HKLM\SOFTWARE\Policies\Google\Chrome" /v DnsOverHttpsMode

# Configurazione salvata
Get-Content "$env:LOCALAPPDATA\SiteBlocker\config.json"
```
