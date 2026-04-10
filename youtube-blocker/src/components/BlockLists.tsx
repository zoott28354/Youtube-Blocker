import { useState, useRef, useCallback } from "react";
import { BlockList } from "../hooks/useBlocker";
import { useI18n } from "../i18n";

interface Props {
  lists: BlockList[];
  blocked: boolean;
  onToggle: (id: string, active: boolean) => Promise<void>;
  onCreate: (name: string) => Promise<BlockList>;
  onUpdate: (id: string, name: string, sites: string[]) => Promise<void>;
  onDelete: (id: string) => Promise<void>;
}

// Prefissi sottodominio comuni (allineati con Rust)
const SUBDOMAIN_PREFIXES = [
  "www", "m",
  "it", "en", "fr", "de", "es", "pt", "nl", "ru", "pl", "tr",
  "ja", "ko", "zh", "ar", "hi", "th", "vi", "id",
  "sv", "da", "no", "fi", "cs", "el", "ro", "hu", "bg", "uk", "hr",
];

// Replica la logica expand_domain di Rust
function expandDomain(input: string): string[] {
  let domain = input
    .trim()
    .toLowerCase()
    .replace(/^https?:\/\//, "")
    .split("/")[0];
  // Rimuove qualsiasi prefisso noto per ottenere il root
  for (const prefix of SUBDOMAIN_PREFIXES) {
    if (domain.startsWith(prefix + ".")) {
      domain = domain.slice(prefix.length + 1);
      break;
    }
  }
  if (!domain || !domain.includes(".")) return [];
  return [domain, ...SUBDOMAIN_PREFIXES.map((p) => `${p}.${domain}`)];
}

// Estrae il root domain rimuovendo qualsiasi prefisso noto
function stripPrefix(site: string): string {
  for (const prefix of SUBDOMAIN_PREFIXES) {
    if (site.startsWith(prefix + ".")) return site.slice(prefix.length + 1);
  }
  return site;
}

// Raggruppa i siti per root domain per mostrarli in modo compatto
function groupByRoot(sites: string[]): Record<string, string[]> {
  const groups: Record<string, string[]> = {};
  for (const site of sites) {
    const root = stripPrefix(site);
    if (!groups[root]) groups[root] = [];
    if (!groups[root].includes(site)) groups[root].push(site);
  }
  return groups;
}

// Estrae solo i root domain per la visualizzazione compatta
function rootDomains(sites: string[]): string[] {
  return Object.keys(groupByRoot(sites));
}

interface EditState {
  id: string;
  name: string;
  sites: string[];
}

export default function BlockLists({
  lists,
  blocked,
  onToggle,
  onCreate,
  onUpdate,
  onDelete,
}: Props) {
  const { t } = useI18n();
  const [editing, setEditing] = useState<EditState | null>(null);
  const [addInput, setAddInput] = useState("");
  const [addError, setAddError] = useState<string | null>(null);
  const [newListName, setNewListName] = useState("");
  const [creating, setCreating] = useState(false);
  const [toggling, setToggling] = useState<string | null>(null);
  const [confirmDeleteId, setConfirmDeleteId] = useState<string | null>(null);
  const [expandedId, setExpandedId] = useState<string | null>(null);
  const addInputRef = useRef<HTMLInputElement>(null);

  const toggleExpand = useCallback((id: string) => {
    setExpandedId((prev) => (prev === id ? null : id));
  }, []);

  function startEdit(list: BlockList) {
    setEditing({ id: list.id, name: list.name, sites: [...list.sites] });
    setAddInput("");
    setAddError(null);
    setConfirmDeleteId(null);
  }

  function cancelEdit() {
    setEditing(null);
    setAddInput("");
    setAddError(null);
  }

  async function saveEdit() {
    if (!editing) return;
    await onUpdate(editing.id, editing.name, editing.sites);
    setEditing(null);
  }

  function addSiteToEdit() {
    if (!editing) return;
    const expanded = expandDomain(addInput);
    if (expanded.length === 0) {
      setAddError("Dominio non valido (usa formato: es. netflix.com)");
      return;
    }
    const newSites = [...editing.sites];
    for (const s of expanded) {
      if (!newSites.includes(s)) newSites.push(s);
    }
    setEditing({ ...editing, sites: newSites });
    setAddInput("");
    setAddError(null);
    addInputRef.current?.focus();
  }

  function removeSiteFromEdit(site: string) {
    if (!editing) return;
    setEditing({
      ...editing,
      sites: editing.sites.filter((s) => s !== site),
    });
  }

  async function handleToggle(id: string, active: boolean) {
    setToggling(id);
    try {
      await onToggle(id, active);
    } finally {
      setToggling(null);
    }
  }

  async function handleDelete(id: string) {
    if (editing?.id === id) setEditing(null);
    setConfirmDeleteId(null);
    await onDelete(id);
  }

  async function handleDuplicate(list: BlockList) {
    const newList = await onCreate(displayName(list) + " " + t.duplicateSuffix);
    await onUpdate(newList.id, newList.name, [...list.sites]);
    // Entra in edit mode sulla nuova lista
    setEditing({ id: newList.id, name: newList.name, sites: [...list.sites] });
    setAddInput("");
    setAddError(null);
    setConfirmDeleteId(null);
  }

  async function handleCreate() {
    const name = newListName.trim();
    if (!name) return;
    setCreating(true);
    try {
      await onCreate(name);
      setNewListName("");
    } finally {
      setCreating(false);
    }
  }

  // Mappa nomi builtin tradotti (i18n)
  const builtinNameMap: Record<string, string> = {
    "builtin-youtube": t.listNameYoutube,
    "builtin-gaming": t.listNameGaming,
    "builtin-platforms": t.listNamePlatforms,
    "builtin-messaging": t.listNameMessaging,
    "builtin-social": t.listNameSocial,
    "builtin-streaming": t.listNameStreaming,
    "migrated-custom": t.listNameCustom,
  };

  function displayName(list: BlockList): string {
    return builtinNameMap[list.id] || list.name;
  }

  function siteCount(sites: string[]) {
    const roots = rootDomains(sites);
    const n = roots.length;
    return `${n} ${n === 1 ? t.siteSingular : t.sitePlural}`;
  }

  // Ordina: custom/duplicate in alto, builtin in basso
  const sortedLists = [...lists].sort((a, b) => {
    if (a.builtin === b.builtin) return 0;
    return a.builtin ? 1 : -1;
  });

  return (
    <div className="space-y-3">
      <p className="text-xs text-gray-500">{t.listsHint}</p>

      {blocked && (
        <p className="text-xs text-amber-400">{t.listsLockedHint}</p>
      )}

      {/* Crea nuova lista */}
      <div className="border-b border-gray-800 pb-3">
        <div className="flex gap-2">
          <input
            className="flex-1 bg-gray-900 border border-gray-700 rounded-lg px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-blue-500"
            value={newListName}
            onChange={(e) => setNewListName(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleCreate()}
            placeholder={t.newListNamePlaceholder}
          />
          <button
            onClick={handleCreate}
            disabled={creating || !newListName.trim()}
            className="px-4 py-2 bg-blue-600 hover:bg-blue-500 disabled:opacity-40 text-white text-sm font-semibold rounded-lg transition-colors"
          >
            {t.createBtn}
          </button>
        </div>
      </div>

      {lists.length === 0 && (
        <p className="text-sm text-gray-500 py-4 text-center">{t.noLists}</p>
      )}

      {sortedLists.map((list) => (
        <div key={list.id} className="bg-gray-900 rounded-xl overflow-hidden">
          {editing?.id === list.id ? (
            /* ── Modalità modifica ── */
            <div className="p-4 space-y-3">
              {/* Nome lista */}
              <input
                className="w-full bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-blue-500"
                value={editing.name}
                onChange={(e) =>
                  setEditing({ ...editing, name: e.target.value })
                }
                placeholder={t.listNamePlaceholder}
              />

              {/* Lista siti */}
              <div className="space-y-1 max-h-48 overflow-y-auto">
                {editing.sites.length === 0 ? (
                  <p className="text-xs text-gray-600 py-2 text-center">
                    {t.noSitesInList}
                  </p>
                ) : (
                  editing.sites.map((site) => (
                    <div
                      key={site}
                      className="flex items-center justify-between gap-3 bg-gray-800 rounded-lg px-3 py-2"
                    >
                      <span className="min-w-0 text-sm text-white break-all">{site}</span>
                      <button
                        onClick={() => removeSiteFromEdit(site)}
                        className="text-xs text-red-400 hover:text-red-300 flex-shrink-0"
                      >
                        {t.removeBtn}
                      </button>
                    </div>
                  ))
                )}
              </div>

              {/* Aggiungi sito */}
              <div className="flex gap-2">
                <input
                  ref={addInputRef}
                  className="flex-1 bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-blue-500"
                  value={addInput}
                  onChange={(e) => {
                    setAddInput(e.target.value);
                    setAddError(null);
                  }}
                  onKeyDown={(e) => e.key === "Enter" && addSiteToEdit()}
                  placeholder={t.addSiteToList}
                />
                <button
                  onClick={addSiteToEdit}
                  className="px-3 py-2 bg-gray-700 hover:bg-gray-600 text-white text-sm rounded-lg transition-colors"
                >
                  {t.addBtn}
                </button>
              </div>
              {addError && (
                <p className="text-xs text-red-400">{addError}</p>
              )}

              {/* Salva / Annulla */}
              <div className="flex gap-2 pt-1">
                <button
                  onClick={saveEdit}
                  className="flex-1 py-2 bg-blue-600 hover:bg-blue-500 text-white text-sm font-semibold rounded-lg transition-colors"
                >
                  {t.saveList}
                </button>
                <button
                  onClick={cancelEdit}
                  className="flex-1 py-2 bg-gray-700 hover:bg-gray-600 text-white text-sm rounded-lg transition-colors"
                >
                  {t.cancelEdit}
                </button>
              </div>
            </div>
          ) : confirmDeleteId === list.id ? (
            /* ── Conferma eliminazione inline ── */
            <div className="flex items-center justify-between px-4 py-3">
              <span className="text-sm text-white">
                {t.confirmDeleteList}
              </span>
              <div className="flex gap-2">
                <button
                  onClick={() => handleDelete(list.id)}
                  className="px-3 py-1.5 bg-red-600 hover:bg-red-500 text-white text-xs font-semibold rounded-lg transition-colors"
                >
                  {t.confirmYes}
                </button>
                <button
                  onClick={() => setConfirmDeleteId(null)}
                  className="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 text-white text-xs rounded-lg transition-colors"
                >
                  {t.confirmNo}
                </button>
              </div>
            </div>
          ) : (
            /* ── Modalità visualizzazione compatta ── */
            <div className="px-4 py-3">
              {/* Riga 1: toggle + nome + conteggio + azioni */}
              <div className="flex items-center gap-3">
                {/* Toggle */}
                <button
                  onClick={() => handleToggle(list.id, !list.active)}
                  disabled={blocked || toggling === list.id}
                  className={`relative h-6 w-10 rounded-full transition-colors flex-shrink-0 ${
                    list.active ? "bg-blue-600" : "bg-gray-700"
                  } ${blocked || toggling === list.id ? "opacity-50 cursor-not-allowed" : ""}`}
                >
                  <span
                    className={`absolute left-1 top-1 h-4 w-4 rounded-full bg-white transition-transform ${
                      list.active ? "translate-x-4" : "translate-x-0"
                    }`}
                  />
                </button>

                {/* Nome + badge + conteggio */}
                <div className="flex-1 min-w-0 flex flex-wrap items-center gap-x-2 gap-y-1">
                  <span className="text-sm font-semibold text-white truncate">
                    {displayName(list)}
                  </span>
                  {list.builtin && (
                    <span className="text-xs text-gray-500 bg-gray-800 px-1.5 py-0.5 rounded whitespace-nowrap">
                      {t.builtinBadge}
                    </span>
                  )}
                  <span className="text-xs text-gray-600 whitespace-nowrap">
                    {siteCount(list.sites)}
                  </span>
                </div>

                {/* Azioni sempre visibili */}
                <div className="flex items-center gap-1 flex-shrink-0">
                  {list.builtin ? (
                    <button
                      onClick={() => handleDuplicate(list)}
                      className="text-xs text-gray-400 hover:text-white transition-colors px-2 py-1 rounded hover:bg-gray-800"
                    >
                      {t.duplicateList}
                    </button>
                  ) : (
                    <>
                      <button
                        onClick={() => startEdit(list)}
                        className="text-xs text-gray-400 hover:text-white transition-colors px-2 py-1 rounded hover:bg-gray-800"
                      >
                        {t.editList}
                      </button>
                      <button
                        onClick={() => setConfirmDeleteId(list.id)}
                        className="text-xs text-red-500 hover:text-red-400 transition-colors px-2 py-1 rounded hover:bg-gray-800"
                      >
                        {t.deleteList}
                      </button>
                    </>
                  )}
                </div>
              </div>

              {/* Riga 2: chevron + anteprima domini root */}
              {list.sites.length > 0 && (
                <div
                  className="mt-1 flex items-center gap-1 cursor-pointer"
                  style={{ marginLeft: "52px" }}
                  onClick={() => toggleExpand(list.id)}
                >
                  <svg
                    className={`w-3 h-3 text-gray-500 transition-transform flex-shrink-0 ${expandedId === list.id ? "rotate-180" : ""}`}
                    fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}
                  >
                    <path strokeLinecap="round" strokeLinejoin="round" d="M19 9l-7 7-7-7" />
                  </svg>
                  <span className="text-xs text-gray-500 truncate">
                    {rootDomains(list.sites).join(", ")}
                  </span>
                </div>
              )}

              {/* Sezione espansa: lista siti */}
              {expandedId === list.id && list.sites.length > 0 && (
                <div
                  className="mt-2 space-y-0.5 max-h-40 overflow-y-auto"
                  style={{ marginLeft: "52px" }}
                >
                  {rootDomains(list.sites).map((root) => (
                    <div key={root} className="text-xs text-gray-400 py-0.5">
                      {root}
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}
        </div>
      ))}

    </div>
  );
}
