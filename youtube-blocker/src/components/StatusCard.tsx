import { BlockStatus } from "../hooks/useBlocker";
import { useI18n } from "../i18n";

interface Props {
  status: BlockStatus | null;
  loading: boolean;
  onBlock: () => void;
  onUnblock: () => void;
}

export default function StatusCard({ status, loading, onBlock, onUnblock }: Props) {
  const { t } = useI18n();
  const hasActiveLists = (status?.active_lists_count ?? 0) > 0;
  const isBlocked = Boolean(
    status &&
      hasActiveLists &&
      status.hosts_blocked &&
      (!status.block_doh_enabled || (status.firewall_active && status.browser_policy))
  );

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
          {isBlocked ? t.blocked : t.unblocked}
        </div>

        {status && (
          <div className="space-y-3 mt-1">
            <div className="flex justify-center gap-4 text-xs text-gray-400">
              {(
                [
                  [t.hosts, status.hosts_blocked],
                  [t.firewallDoh, status.firewall_active],
                  [t.policyBrowser, status.browser_policy],
                ] as [string, boolean][]
              ).map(([label, active]) => (
                <span key={label}>
                  <span className={active ? "text-red-400" : "text-gray-500"}>●</span>{" "}
                  {label}
                </span>
              ))}
            </div>

            <div className="text-sm text-gray-300">
              <span className="text-gray-400">{t.activeListsLabel}: </span>
              {status.active_list_names.length > 0
                ? status.active_list_names.join(", ")
                : t.noActiveLists}
            </div>
          </div>
        )}
      </div>


      {/* Bottoni azione */}
      <div className="flex gap-4 mb-2">
        <button
          onClick={onBlock}
          disabled={loading || isBlocked}
          className="flex-1 py-3.5 rounded-xl font-semibold text-sm
            bg-red-700 hover:bg-red-600
            disabled:opacity-40 disabled:cursor-not-allowed
            transition-colors"
        >
          {loading && isBlocked ? "..." : t.blockBtn}
        </button>
        <button
          onClick={onUnblock}
          disabled={loading || !isBlocked}
          className="flex-1 py-3.5 rounded-xl font-semibold text-sm
            bg-green-700 hover:bg-green-600
            disabled:opacity-40 disabled:cursor-not-allowed
            transition-colors"
        >
          {loading && !isBlocked ? "..." : t.unblockBtn}
        </button>
      </div>
      {/* Icona decorativa */}
      <div className="flex justify-center pt-2">
        <img
          src="/icon.png"
          alt=""
          className={`w-40 h-40 transition-all duration-500 ${
            isBlocked
              ? "opacity-100 drop-shadow-[0_0_24px_rgba(248,113,113,0.5)]"
              : "opacity-20 grayscale"
          }`}
        />
      </div>
    </div>
  );
}
