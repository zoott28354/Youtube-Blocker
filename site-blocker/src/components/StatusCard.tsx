import { BlockStatus } from "../hooks/useBlocker";

interface Props {
  status: BlockStatus | null;
  loading: boolean;
  onBlock: () => void;
  onUnblock: () => void;
}

export default function StatusCard({ status, loading, onBlock, onUnblock }: Props) {
  const isBlocked = status?.hosts_blocked ?? false;

  return (
    <div className="space-y-5">
      {/* Badge stato */}
      <div
        className={`rounded-2xl p-10 text-center border transition-all duration-300 ${
          isBlocked
            ? "bg-red-950/60 border-red-800"
            : "bg-green-950/60 border-green-800"
        }`}
      >
        <div
          className={`text-4xl font-black tracking-widest mb-3 ${
            isBlocked ? "text-red-400" : "text-green-400"
          }`}
        >
          {isBlocked ? "BLOCCATO" : "SBLOCCATO"}
        </div>

        {status && (
          <div className="flex justify-center gap-4 text-xs text-gray-400 mt-1">
            <span>
              <span
                className={status.hosts_blocked ? "text-red-400" : "text-gray-500"}
              >
                ●
              </span>{" "}
              Hosts
            </span>
            <span>
              <span
                className={
                  status.firewall_active ? "text-red-400" : "text-gray-500"
                }
              >
                ●
              </span>{" "}
              Firewall DoH
            </span>
          </div>
        )}
      </div>

      {/* Avviso riavvio browser */}
      {isBlocked && (
        <p className="text-xs text-yellow-500/80 text-center -mt-1">
          Riavvia i browser già aperti per applicare il blocco subito.
        </p>
      )}

      {/* Bottoni azione */}
      <div className="flex gap-4">
        <button
          onClick={onBlock}
          disabled={loading || isBlocked}
          className="flex-1 py-3.5 rounded-xl font-semibold text-sm
            bg-red-700 hover:bg-red-600
            disabled:opacity-40 disabled:cursor-not-allowed
            transition-colors"
        >
          {loading && isBlocked ? "..." : "🔒  Blocca"}
        </button>
        <button
          onClick={onUnblock}
          disabled={loading || !isBlocked}
          className="flex-1 py-3.5 rounded-xl font-semibold text-sm
            bg-green-700 hover:bg-green-600
            disabled:opacity-40 disabled:cursor-not-allowed
            transition-colors"
        >
          {loading && !isBlocked ? "..." : "🔓  Sblocca"}
        </button>
      </div>
    </div>
  );
}
