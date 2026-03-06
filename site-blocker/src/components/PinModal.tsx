import { useState, useRef, useEffect } from "react";

interface Props {
  mode: "setup" | "verify";
  onConfirm: (pin: string) => Promise<void>;
  onCancel?: () => void;
}

export default function PinModal({ mode, onConfirm, onCancel }: Props) {
  const [pin, setPin] = useState("");
  const [confirm, setConfirm] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  const handleSubmit = async () => {
    setError(null);
    if (pin.length < 4) {
      setError("Il PIN deve avere almeno 4 caratteri");
      return;
    }
    if (mode === "setup" && pin !== confirm) {
      setError("I PIN non coincidono");
      return;
    }
    setSubmitting(true);
    try {
      await onConfirm(pin);
    } catch (e) {
      const msg = String(e);
      setError(msg.includes("PIN errato") ? "PIN errato" : msg);
      setPin("");
      if (mode === "verify") setConfirm("");
    } finally {
      setSubmitting(false);
    }
  };

  const inner = (
    <div className="bg-gray-900 border border-gray-700 rounded-2xl p-6 w-full max-w-xs shadow-2xl">
      <h2 className="text-lg font-bold mb-1">
        {mode === "setup" ? "Imposta PIN genitore" : "PIN richiesto"}
      </h2>
      <p className="text-sm text-gray-400 mb-4">
        {mode === "setup"
          ? "Questo PIN sarà richiesto per sbloccare i siti."
          : "Inserisci il PIN per sbloccare."}
      </p>

      <input
        ref={inputRef}
        type="password"
        inputMode="numeric"
        placeholder="PIN"
        value={pin}
        onChange={(e) => setPin(e.target.value)}
        onKeyDown={(e) => e.key === "Enter" && handleSubmit()}
        className="w-full bg-gray-800 border border-gray-600 rounded-xl
          px-4 py-3 text-center text-xl tracking-widest mb-3 outline-none
          focus:border-blue-500 transition-colors"
      />

      {mode === "setup" && (
        <input
          type="password"
          inputMode="numeric"
          placeholder="Conferma PIN"
          value={confirm}
          onChange={(e) => setConfirm(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleSubmit()}
          className="w-full bg-gray-800 border border-gray-600 rounded-xl
            px-4 py-3 text-center text-xl tracking-widest mb-3 outline-none
            focus:border-blue-500 transition-colors"
        />
      )}

      {error && (
        <p className="text-red-400 text-sm mb-3 text-center font-medium">
          {error}
        </p>
      )}

      <div className="flex gap-3">
        {onCancel && (
          <button
            onClick={onCancel}
            disabled={submitting}
            className="flex-1 py-2.5 rounded-xl bg-gray-700 hover:bg-gray-600
              disabled:opacity-50 font-medium transition-colors"
          >
            Annulla
          </button>
        )}
        <button
          onClick={handleSubmit}
          disabled={submitting}
          className="flex-1 py-2.5 rounded-xl bg-blue-600 hover:bg-blue-500
            disabled:opacity-50 font-semibold transition-colors"
        >
          {submitting ? "..." : mode === "setup" ? "Imposta" : "Sblocca"}
        </button>
      </div>
    </div>
  );

  if (onCancel) {
    return (
      <div className="fixed inset-0 bg-black/70 flex items-center justify-center z-50 p-4">
        {inner}
      </div>
    );
  }

  return <div className="flex items-center justify-center p-4">{inner}</div>;
}
