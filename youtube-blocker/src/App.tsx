import { useState } from "react";
import { useBlocker } from "./hooks/useBlocker";
import { useI18n, Lang, LANG_LABELS } from "./i18n";
import StatusCard from "./components/StatusCard";
import BlockLists from "./components/BlockLists";
import PinModal from "./components/PinModal";
import Settings from "./components/Settings";
import About from "./components/About";

type Tab = "main" | "lists" | "settings" | "about";

export default function App() {
  const blocker = useBlocker();
  const { lang, setLang, t } = useI18n();
  const [tab, setTab] = useState<Tab>("main");
  const [showPinModal, setShowPinModal] = useState(false);

  const tabs: { key: Tab; label: string }[] = [
    { key: "main", label: t.tabs.main },
    { key: "lists", label: t.tabs.lists },
    { key: "settings", label: t.tabs.settings },
    { key: "about", label: t.tabs.about },
  ];

  if (blocker.hasPinSet === null) {
    return (
      <div className="min-h-screen flex items-center justify-center text-gray-400 text-sm">
        {t.loading}
      </div>
    );
  }

  if (!blocker.hasPinSet) {
    return (
      <div className="min-h-screen bg-gray-950 flex flex-col items-center justify-center p-6">
        <div className="mb-6 text-center">
          <h1 className="text-2xl font-black text-white tracking-tight">
            {t.title}
          </h1>
          <p className="text-sm text-gray-400 mt-1">{t.setupPinHint}</p>
        </div>
        <PinModal mode="setup" onConfirm={blocker.setPin} />
      </div>
    );
  }

  // PIN impostato ma sessione non ancora autenticata
  if (!blocker.isSessionUnlocked) {
    return (
      <div className="min-h-screen bg-gray-950 flex flex-col items-center justify-center p-6">
        <div className="mb-6 text-center">
          <h1 className="text-2xl font-black text-white tracking-tight">
            {t.title}
          </h1>
        </div>
        <PinModal mode="verify" onConfirm={blocker.sessionUnlock} />
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-950 flex flex-col">
      {/* Header */}
      <header className="px-5 pt-5 pb-3 flex items-center justify-between">
        <h1 className="text-xl font-black text-white tracking-tight">
          {t.title}
        </h1>
        <div className="flex items-center gap-2">
          {blocker.error && (
            <span className="text-xs text-red-400 truncate max-w-[160px]">
              {blocker.error}
            </span>
          )}
          {/* Language selector */}
          <select
            value={lang}
            onChange={(e) => setLang(e.target.value as Lang)}
            className="bg-gray-800 border border-gray-700 rounded-lg px-2.5 py-1 text-xs
              font-semibold text-gray-300 uppercase cursor-pointer
              focus:outline-none focus:border-blue-500"
          >
            {(Object.keys(LANG_LABELS) as Lang[]).map((l) => (
              <option key={l} value={l}>
                {LANG_LABELS[l]}
              </option>
            ))}
          </select>
        </div>
      </header>

      {/* Tab bar */}
      <nav className="flex border-b border-gray-800 px-5">
        {tabs.map(({ key, label }) => (
          <button
            key={key}
            onClick={() => setTab(key)}
            className={`px-4 py-2.5 text-sm font-medium transition-colors border-b-2 -mb-px ${
              tab === key
                ? "text-white border-blue-500"
                : "text-gray-400 border-transparent hover:text-gray-200"
            }`}
          >
            {label}
          </button>
        ))}
      </nav>

      {/* Contenuto */}
      <main className="flex-1 p-5 overflow-y-auto">
        {tab === "main" && (
          <StatusCard
            status={blocker.status}
            loading={blocker.loading}
            onBlock={blocker.blockAll}
            onUnblock={() => setShowPinModal(true)}
          />
        )}
        {tab === "lists" && (
          <BlockLists
            lists={blocker.lists}
            blocked={Boolean(blocker.status && blocker.status.hosts_blocked)}
            onToggle={blocker.toggleList}
            onCreate={blocker.createList}
            onUpdate={blocker.updateList}
            onDelete={blocker.deleteList}
          />
        )}
        {tab === "settings" && (
          <Settings onChangePin={blocker.changePin} onResetPin={blocker.resetPin} />
        )}
        {tab === "about" && <About />}
      </main>

      {showPinModal && (
        <PinModal
          mode="verify"
          onConfirm={async (pin) => {
            await blocker.unblockAll(pin);
            setShowPinModal(false);
          }}
          onCancel={() => setShowPinModal(false)}
        />
      )}
    </div>
  );
}
