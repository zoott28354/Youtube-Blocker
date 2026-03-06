import { useState } from "react";

interface Props {
  sites: string[];
  onAdd: (domain: string) => Promise<void>;
  onRemove: (domain: string) => Promise<void>;
}

export default function SiteList({ sites, onAdd, onRemove }: Props) {
  const [input, setInput] = useState("");
  const [adding, setAdding] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleAdd = async () => {
    const domain = input.trim().toLowerCase();
    if (!domain) return;
    setError(null);
    setAdding(true);
    try {
      await onAdd(domain);
      setInput("");
    } catch (e) {
      setError(String(e));
    } finally {
      setAdding(false);
    }
  };

  return (
    <div className="space-y-4">
      <div>
        <h2 className="text-base font-bold text-gray-100">Siti bloccati</h2>
        <p className="text-xs text-gray-400 mt-0.5">
          Modifiche attive al prossimo blocco.
        </p>
      </div>

      {/* Input aggiunta */}
      <div className="flex gap-2">
        <input
          type="text"
          placeholder="es. tiktok.com"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleAdd()}
          className="flex-1 bg-gray-800 border border-gray-700 rounded-xl
            px-4 py-2.5 text-sm outline-none focus:border-blue-500 transition-colors"
        />
        <button
          onClick={handleAdd}
          disabled={adding || !input.trim()}
          className="px-4 py-2.5 bg-blue-600 hover:bg-blue-500 rounded-xl
            disabled:opacity-50 text-sm font-semibold transition-colors"
        >
          Aggiungi
        </button>
      </div>

      {error && <p className="text-red-400 text-xs">{error}</p>}

      {/* Lista */}
      <ul className="space-y-2">
        {sites.map((site) => (
          <li
            key={site}
            className="flex items-center justify-between bg-gray-900
              border border-gray-800 rounded-xl px-4 py-3 group"
          >
            <span className="font-mono text-sm text-gray-300">{site}</span>
            <button
              onClick={() => onRemove(site)}
              className="text-gray-500 hover:text-red-400 text-xs font-medium
                opacity-0 group-hover:opacity-100 transition-all"
            >
              Rimuovi
            </button>
          </li>
        ))}
        {sites.length === 0 && (
          <li className="text-center text-sm text-gray-500 py-6">
            Nessun sito nella lista.
          </li>
        )}
      </ul>
    </div>
  );
}
