import { useState } from "react";

interface Props {
  onChangePin: (oldPin: string, newPin: string) => Promise<void>;
}

export default function Settings({ onChangePin }: Props) {
  const [oldPin, setOldPin] = useState("");
  const [newPin, setNewPin] = useState("");
  const [confirmPin, setConfirmPin] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);
  const [submitting, setSubmitting] = useState(false);

  const handleChange = async () => {
    setError(null);
    setSuccess(false);
    if (newPin.length < 4) {
      setError("Il PIN deve avere almeno 4 caratteri");
      return;
    }
    if (newPin !== confirmPin) {
      setError("I PIN non coincidono");
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
      setError(msg.includes("PIN errato") ? "PIN attuale errato" : msg);
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="space-y-6 max-w-sm">
      <div>
        <h2 className="text-base font-bold text-gray-100">Impostazioni</h2>
        <p className="text-xs text-gray-400 mt-0.5">Gestisci il PIN di sblocco.</p>
      </div>

      <div className="bg-gray-900 border border-gray-800 rounded-2xl p-5 space-y-3">
        <h3 className="font-semibold text-gray-200 text-sm">Cambia PIN</h3>

        {[
          { label: "PIN attuale", value: oldPin, set: setOldPin },
          { label: "Nuovo PIN", value: newPin, set: setNewPin },
          { label: "Conferma nuovo PIN", value: confirmPin, set: setConfirmPin },
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
        {success && <p className="text-green-400 text-xs">PIN aggiornato con successo.</p>}

        <button
          onClick={handleChange}
          disabled={submitting}
          className="w-full py-2.5 bg-blue-600 hover:bg-blue-500
            rounded-xl font-semibold text-sm disabled:opacity-50 transition-colors"
        >
          {submitting ? "..." : "Aggiorna PIN"}
        </button>
      </div>
    </div>
  );
}
