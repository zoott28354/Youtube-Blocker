import { createContext, useContext, useState, ReactNode } from "react";

export type Lang = "it" | "en";

export const translations = {
  it: {
    title: "YouTube Blocker",
    loading: "Caricamento...",
    setupPinHint: "Configura il PIN prima di iniziare.",
    tabs: { main: "Stato", lists: "Liste", settings: "Impostazioni", about: "Info" },
    // StatusCard
    blocked: "BLOCCATO",
    unblocked: "SBLOCCATO",
    hosts: "Hosts",
    firewallDoh: "Firewall DoH",
    policyBrowser: "Policy browser",
    blockBtn: "🔒  Blocca",
    unblockBtn: "🔓  Sblocca",
    // BlockLists
    listsTitle: "Le mie liste",
    listsHint: "Attiva le liste da bloccare, poi premi Blocca nella tab Stato.",
    newList: "+ Nuova lista",
    newListNamePlaceholder: "Nome nuova lista...",
    createBtn: "Crea",
    editList: "Modifica",
    deleteList: "Elimina",
    confirmDeleteList: "Eliminare questa lista?",
    noLists: "Nessuna lista. Creane una nuova.",
    saveList: "Salva",
    cancelEdit: "Annulla",
    listNamePlaceholder: "Nome lista",
    builtinBadge: "predefinita",
    addSiteToList: "es. netflix.com",
    addBtn: "Aggiungi",
    removeBtn: "Rimuovi",
    noSitesInList: "Nessun sito in questa lista.",
    siteSingular: "sito",
    sitePlural: "siti",
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
    // About
    aboutDescription: "App desktop per bloccare siti a livello di sistema. Pensata per genitori che vogliono limitare l'accesso a YouTube e altri siti per i propri figli.",
    aboutLicense: "Licenza MIT",
    aboutViewOnGithub: "Vedi su GitHub",
  },
  en: {
    title: "YouTube Blocker",
    loading: "Loading...",
    setupPinHint: "Set up your PIN before getting started.",
    tabs: { main: "Status", lists: "Lists", settings: "Settings", about: "About" },
    // StatusCard
    blocked: "BLOCKED",
    unblocked: "UNBLOCKED",
    hosts: "Hosts",
    firewallDoh: "Firewall DoH",
    policyBrowser: "Browser Policy",
    blockBtn: "🔒  Block",
    unblockBtn: "🔓  Unblock",
    // BlockLists
    listsTitle: "My lists",
    listsHint: "Enable the lists to block, then press Block in the Status tab.",
    newList: "+ New list",
    newListNamePlaceholder: "New list name...",
    createBtn: "Create",
    editList: "Edit",
    deleteList: "Delete",
    confirmDeleteList: "Delete this list?",
    noLists: "No lists. Create a new one.",
    saveList: "Save",
    cancelEdit: "Cancel",
    listNamePlaceholder: "List name",
    builtinBadge: "built-in",
    addSiteToList: "e.g. netflix.com",
    addBtn: "Add",
    removeBtn: "Remove",
    noSitesInList: "No sites in this list.",
    siteSingular: "site",
    sitePlural: "sites",
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
    // About
    aboutDescription: "Desktop app to block websites at system level. Built for parents who want to limit their children's access to YouTube and other sites.",
    aboutLicense: "MIT License",
    aboutViewOnGithub: "View on GitHub",
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
