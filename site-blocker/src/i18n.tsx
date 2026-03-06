import { createContext, useContext, useState, ReactNode } from "react";

export type Lang = "it" | "en";

export const translations = {
  it: {
    title: "YouTube Blocker",
    loading: "Caricamento...",
    setupPinHint: "Configura il PIN prima di iniziare.",
    tabs: { main: "Stato", sites: "Siti", settings: "Impostazioni" },
    // StatusCard
    blocked: "BLOCCATO",
    unblocked: "SBLOCCATO",
    hosts: "Hosts",
    firewallDoh: "Firewall DoH",
    policyBrowser: "Policy browser",
    blockBtn: "🔒  Blocca",
    unblockBtn: "🔓  Sblocca",
    // SiteList
    sitesTitle: "Siti bloccati",
    sitesHint: "Modifiche attive al prossimo blocco. Le varianti www e m. vengono aggiunte automaticamente.",
    addDefaults: "+ YouTube predefiniti",
    addPlaceholder: "es. netflix.com (con .com)",
    addBtn: "Aggiungi",
    removeBtn: "Rimuovi",
    noSites: "Nessun sito nella lista.",
    // PinModal
    pinSetupTitle: "Imposta PIN genitore",
    pinVerifyTitle: "PIN richiesto",
    pinSetupHint: "Questo PIN sarà richiesto per sbloccare i siti.",
    pinVerifyHint: "Inserisci il PIN per sbloccare.",
    pinTooShort: "Il PIN deve avere almeno 4 caratteri",
    pinMismatch: "I PIN non coincidono",
    pinWrong: "PIN errato",
    pinConfirmPlaceholder: "Conferma PIN",
    cancel: "Annulla",
    pinSetupBtn: "Imposta",
    pinUnlockBtn: "Sblocca",
    // Settings
    settingsTitle: "Impostazioni",
    settingsHint: "Gestisci il PIN di sblocco.",
    changePinTitle: "Cambia PIN",
    currentPin: "PIN attuale",
    newPin: "Nuovo PIN",
    confirmNewPin: "Conferma nuovo PIN",
    pinCurrentWrong: "PIN attuale errato",
    pinUpdated: "PIN aggiornato con successo.",
    updatePinBtn: "Aggiorna PIN",
    // Reset PIN
    resetPinTitle: "Reset PIN",
    resetPinHint: "Rimuove il PIN attuale. Al prossimo avvio verrà richiesto di impostarne uno nuovo.",
    resetPinBtn: "Reset PIN",
    resetPinConfirm: "Sei sicuro? Il PIN verrà rimosso.",
    resetPinDone: "PIN rimosso. Riavvia l'app per impostarne uno nuovo.",
  },
  en: {
    title: "YouTube Blocker",
    loading: "Loading...",
    setupPinHint: "Set up your PIN before getting started.",
    tabs: { main: "Status", sites: "Sites", settings: "Settings" },
    // StatusCard
    blocked: "BLOCKED",
    unblocked: "UNBLOCKED",
    hosts: "Hosts",
    firewallDoh: "Firewall DoH",
    policyBrowser: "Browser Policy",
    blockBtn: "🔒  Block",
    unblockBtn: "🔓  Unblock",
    // SiteList
    sitesTitle: "Blocked sites",
    sitesHint: "Changes take effect on next block. www and m. variants are added automatically.",
    addDefaults: "+ YouTube defaults",
    addPlaceholder: "e.g. netflix.com (with .com)",
    addBtn: "Add",
    removeBtn: "Remove",
    noSites: "No sites in the list.",
    // PinModal
    pinSetupTitle: "Set parent PIN",
    pinVerifyTitle: "PIN required",
    pinSetupHint: "This PIN will be required to unblock sites.",
    pinVerifyHint: "Enter your PIN to unblock.",
    pinTooShort: "PIN must be at least 4 characters",
    pinMismatch: "PINs do not match",
    pinWrong: "Wrong PIN",
    pinConfirmPlaceholder: "Confirm PIN",
    cancel: "Cancel",
    pinSetupBtn: "Set",
    pinUnlockBtn: "Unlock",
    // Settings
    settingsTitle: "Settings",
    settingsHint: "Manage your unlock PIN.",
    changePinTitle: "Change PIN",
    currentPin: "Current PIN",
    newPin: "New PIN",
    confirmNewPin: "Confirm new PIN",
    pinCurrentWrong: "Current PIN is incorrect",
    pinUpdated: "PIN updated successfully.",
    updatePinBtn: "Update PIN",
    // Reset PIN
    resetPinTitle: "Reset PIN",
    resetPinHint: "Removes the current PIN. You will be prompted to set a new one on next launch.",
    resetPinBtn: "Reset PIN",
    resetPinConfirm: "Are you sure? The PIN will be removed.",
    resetPinDone: "PIN removed. Restart the app to set a new one.",
  },
} as const;

type Translations = (typeof translations)[Lang];

const LangContext = createContext<{
  lang: Lang;
  setLang: (l: Lang) => void;
  t: Translations;
}>({ lang: "it", setLang: () => {}, t: translations.it });

export function LangProvider({ children }: { children: ReactNode }) {
  const stored = (localStorage.getItem("lang") as Lang) ?? "it";
  const [lang, setLangState] = useState<Lang>(stored);

  const setLang = (l: Lang) => {
    localStorage.setItem("lang", l);
    setLangState(l);
  };

  return (
    <LangContext.Provider value={{ lang, setLang, t: translations[lang] }}>
      {children}
    </LangContext.Provider>
  );
}

export function useI18n() {
  return useContext(LangContext);
}
