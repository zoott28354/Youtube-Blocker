import { useState } from "react";
import { useBlocker } from "./hooks/useBlocker";
import StatusCard from "./components/StatusCard";
import SiteList from "./components/SiteList";
import PinModal from "./components/PinModal";
import Settings from "./components/Settings";

type Tab = "main" | "sites" | "settings";

const TAB_LABELS: Record<Tab, string> = {
  main: "Stato",
  sites: "Siti",
  settings: "Impostazioni",
};

export default function App() {
  const blocker = useBlocker();
  const [tab, setTab] = useState<Tab>("main");
  const [showPinModal, setShowPinModal] = useState(false);

  // Loading iniziale
  if (blocker.hasPinSet === null) {
    return (
      <div className="min-h-screen flex items-center justify-center text-gray-400 text-sm">
        Caricamento...
      </div>
    );
  }

  // Primo avvio: PIN non ancora impostato
  if (!blocker.hasPinSet) {
    return (
      <div className="min-h-screen bg-gray-950 flex flex-col items-center justify-center p-6">
        <div className="mb-6 text-center">
          <h1 className="text-2xl font-black text-white tracking-tight">
            SiteBlocker
          </h1>
          <p className="text-sm text-gray-400 mt-1">
            Configura il PIN prima di iniziare.
          </p>
        </div>
        <PinModal mode="setup" onConfirm={blocker.setPin} />
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-950 flex flex-col">
      {/* Header */}
      <header className="px-5 pt-5 pb-3 flex items-center justify-between">
        <h1 className="text-lg font-black text-white tracking-tight">
          SiteBlocker
        </h1>
        {blocker.error && (
          <span className="text-xs text-red-400 truncate max-w-xs">
            {blocker.error}
          </span>
        )}
      </header>

      {/* Tab bar */}
      <nav className="flex border-b border-gray-800 px-5">
        {(Object.keys(TAB_LABELS) as Tab[]).map((t) => (
          <button
            key={t}
            onClick={() => setTab(t)}
            className={`px-4 py-2.5 text-sm font-medium transition-colors border-b-2 -mb-px ${
              tab === t
                ? "text-white border-blue-500"
                : "text-gray-400 border-transparent hover:text-gray-200"
            }`}
          >
            {TAB_LABELS[t]}
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
          <Settings onChangePin={blocker.changePin} />
        )}
      </main>

      {/* Modal PIN sblocco */}
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
