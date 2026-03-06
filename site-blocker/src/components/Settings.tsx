import { useState } from "react";
import { useI18n } from "../i18n";

interface Props {
  onChangePin: (oldPin: string, newPin: string) => Promise<void>;
  onResetPin: () => Promise<void>;
}

export default function Settings({ onChangePin, onResetPin }: Props) {
  const { t } = useI18n();
  const [oldPin, setOldPin] = useState("");
  const [newPin, setNewPin] = useState("");
  const [confirmPin, setConfirmPin] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);
  const [submitting, setSubmitting] = useState(false);

  const [confirmReset, setConfirmReset] = useState(false);
  const [resetDone, setResetDone] = useState(false);

  const handleChange = async () => {
    setError(null);
    setSuccess(false);
    if (newPin.length < 4) {
      setError(t.pinTooShort);
      return;
    }
    if (newPin !== confirmPin) {
      setError(t.pinMismatch);
      return;
    }
    setSubmitting(true);
    try {
      await onChangePin(oldPin, newPin);
      setOldPin("");
      setNewPin("");
      setConfirmPin("");
      setSuccess(true);
    } catch (e) {
      const msg = String(e);
      setError(msg.includes("PIN errato") || msg.includes("Wrong PIN") ? t.pinCurrentWrong : msg);
    } finally {
      setSubmitting(false);
    }
  };

  const handleReset = async () => {
    await onResetPin();
    setConfirmReset(false);
    setResetDone(true);
  };

  return (
    <div className="space-y-6 max-w-sm">
      <div>
        <h2 className="text-base font-bold text-gray-100">{t.settingsTitle}</h2>
        <p className="text-xs text-gray-400 mt-0.5">{t.settingsHint}</p>
      </div>

      {/* Cambia PIN */}
      <div className="bg-gray-900 border border-gray-800 rounded-2xl p-5 space-y-3">
        <h3 className="font-semibold text-gray-200 text-sm">{t.changePinTitle}</h3>

        {[
          { label: t.currentPin, value: oldPin, set: setOldPin },
          { label: t.newPin, value: newPin, set: setNewPin },
          { label: t.confirmNewPin, value: confirmPin, set: setConfirmPin },
        ].map(({ label, value, set }) => (
          <input
            key={label}
            type="password"
            placeholder={label}
            value={value}
            onChange={(e) => set(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleChange()}
            className="w-full bg-gray-800 border border-gray-700 rounded-xl
              px-4 py-2.5 text-sm outline-none focus:border-blue-500 transition-colors"
          />
        ))}

        {error && <p className="text-red-400 text-xs">{error}</p>}
        {success && <p className="text-green-400 text-xs">{t.pinUpdated}</p>}

        <button
          onClick={handleChange}
          disabled={submitting}
          className="w-full py-2.5 bg-blue-600 hover:bg-blue-500
            rounded-xl font-semibold text-sm disabled:opacity-50 transition-colors"
        >
          {submitting ? "..." : t.updatePinBtn}
        </button>
      </div>

      {/* Reset PIN */}
      <div className="bg-gray-900 border border-gray-800 rounded-2xl p-5 space-y-3">
        <h3 className="font-semibold text-gray-200 text-sm">{t.resetPinTitle}</h3>
        <p className="text-xs text-gray-400">{t.resetPinHint}</p>

        {resetDone ? (
          <p className="text-green-400 text-xs">{t.resetPinDone}</p>
        ) : confirmReset ? (
          <div className="space-y-2">
            <p className="text-yellow-400 text-xs font-medium">{t.resetPinConfirm}</p>
            <div className="flex gap-2">
              <button
                onClick={() => setConfirmReset(false)}
                className="flex-1 py-2 rounded-xl bg-gray-700 hover:bg-gray-600
                  text-sm font-medium transition-colors"
              >
                {t.cancel}
              </button>
              <button
                onClick={handleReset}
                className="flex-1 py-2 rounded-xl bg-red-700 hover:bg-red-600
                  text-sm font-semibold transition-colors"
              >
                {t.resetPinBtn}
              </button>
            </div>
          </div>
        ) : (
          <button
            onClick={() => setConfirmReset(true)}
            className="w-full py-2.5 bg-gray-700 hover:bg-gray-600 border border-gray-600
              rounded-xl font-semibold text-sm text-red-400 hover:text-red-300 transition-colors"
          >
            {t.resetPinBtn}
          </button>
        )}
      </div>
    </div>
  );
}
