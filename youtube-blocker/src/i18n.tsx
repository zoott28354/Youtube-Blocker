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
    notAvailableShort: "N/D",
    hostsOnlyMode: "Su {os} il blocco usa al momento il file hosts. Firewall DoH e policy browser non sono ancora disponibili.",
    activeListsLabel: "Liste attive",
    noActiveLists: "Nessuna lista attiva",
    blockBtn: "🔒  Blocca",
    unblockBtn: "🔓  Sblocca",
    // BlockLists
    listsTitle: "Le mie liste",
    listsHint: "Attiva le liste da bloccare, poi premi Blocca nella tab Stato, oppure duplica una lista per farne una personalizzata.",
    newList: "+ Nuova lista",
    newListNamePlaceholder: "Nome nuova lista...",
    createBtn: "Crea",
    editList: "Modifica",
    deleteList: "Elimina",
    confirmDeleteList: "Eliminare?",
    confirmYes: "Si",
    confirmNo: "No",
    duplicateList: "Duplica",
    noLists: "Nessuna lista. Creane una nuova.",
    saveList: "Salva",
    cancelEdit: "Annulla",
    listNamePlaceholder: "Nome lista",
    builtinBadge: "predefinita",
    listNameYoutube: "YouTube & Video",
    listNameGaming: "Browser Games",
    listNamePlatforms: "Gaming Platforms",
    listNameMessaging: "Chat & Messaging",
    listNameSocial: "Social Media",
    listNameStreaming: "Streaming",
    listNameCustom: "Siti personalizzati",
    addSiteToList: "es. netflix.com",
    addBtn: "Aggiungi",
    removeBtn: "Rimuovi",
    noSitesInList: "Nessun sito in questa lista.",
    siteSingular: "sito",
    sitePlural: "siti",
    siteVariantsSuffix: "varianti",
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
    aboutDescription: "App desktop per bloccare siti a livello di sistema. Pensata per genitori che vogliono limitare l'accesso a YouTube e altri siti per i propri figli. Organizza i siti in liste di blocco nominate, con liste predefinite per YouTube, giochi, social e streaming.",
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
    notAvailableShort: "N/A",
    hostsOnlyMode: "On {os}, blocking currently relies on the hosts file. Firewall DoH and browser policy are not available yet.",
    activeListsLabel: "Active lists",
    noActiveLists: "No active lists",
    blockBtn: "🔒  Block",
    unblockBtn: "🔓  Unblock",
    // BlockLists
    listsTitle: "My lists",
    listsHint: "Enable the lists to block, then press Block in the Status tab, or duplicate a list to turn it into a custom one.",
    newList: "+ New list",
    newListNamePlaceholder: "New list name...",
    createBtn: "Create",
    editList: "Edit",
    deleteList: "Delete",
    confirmDeleteList: "Delete?",
    confirmYes: "Yes",
    confirmNo: "No",
    duplicateList: "Duplicate",
    noLists: "No lists. Create a new one.",
    saveList: "Save",
    cancelEdit: "Cancel",
    listNamePlaceholder: "List name",
    builtinBadge: "built-in",
    listNameYoutube: "YouTube & Video",
    listNameGaming: "Browser Games",
    listNamePlatforms: "Gaming Platforms",
    listNameMessaging: "Chat & Messaging",
    listNameSocial: "Social Media",
    listNameStreaming: "Streaming",
    listNameCustom: "Custom sites",
    addSiteToList: "e.g. netflix.com",
    addBtn: "Add",
    removeBtn: "Remove",
    noSitesInList: "No sites in this list.",
    siteSingular: "site",
    sitePlural: "sites",
    siteVariantsSuffix: "variants",
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
    aboutDescription: "Desktop app to block websites at system level. Built for parents who want to limit their children's access to YouTube and other sites. Organize sites into named block lists, with built-in lists for YouTube, games, social media and streaming.",
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
