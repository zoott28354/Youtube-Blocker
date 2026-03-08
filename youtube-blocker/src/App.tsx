import { useState } from "react";
import { useBlocker } from "./hooks/useBlocker";
import { useI18n, Lang } from "./i18n";
import StatusCard from "./components/StatusCard";
import SiteList from "./components/SiteList";
import PinModal from "./components/PinModal";
import Settings from "./components/Settings";
import About from "./components/About";

type Tab = "main" | "sites" | "settings" | "about";

export default function App() {
  const blocker = useBlocker();
  const { lang, setLang, t } = useI18n();
  const [tab, setTab] = useState<Tab>("main");
  const [showPinModal, setShowPinModal] = useState(false);

  const tabs: { key: Tab; label: string }[] = [
    { key: "main", label: t.tabs.main },
    { key: "sites", label: t.tabs.sites },
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
          {/* Language toggle */}
          <div className="flex rounded-lg overflow-hidden border border-gray-700">
            {(["it", "en"] as Lang[]).map((l) => (
              <button
                key={l}
                onClick={() => setLang(l)}
                className={`px-2.5 py-1 text-xs font-semibold uppercase transition-colors ${
                  lang === l
                    ? "bg-blue-600 text-white"
                    : "text-gray-400 hover:text-gray-200"
                }`}
              >
                {l}
              </button>
            ))}
          </div>
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
        {tab === "sites" && (
          <SiteList
            sites={blocker.sites}
            onAdd={blocker.addSite}
            onRemove={blocker.removeSite}
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
