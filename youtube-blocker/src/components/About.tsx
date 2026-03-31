import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from "../i18n";

export default function About() {
  const { t } = useI18n();
  const [version, setVersion] = useState("...");

  useEffect(() => {
    invoke<string>("get_app_version").then(setVersion).catch(() => setVersion("?"));
  }, []);

  return (
    <div className="flex flex-col items-center text-center gap-8 py-8">
      {/* Icona + nome */}
      <div>
        <img
          src="/icon.png"
          alt="YouTube Blocker"
          className="w-20 h-20 mx-auto mb-3 rounded-2xl"
        />
        <h2 className="text-2xl font-black text-white">YouTube Blocker</h2>
        <p className="text-sm text-gray-400 mt-1">v{version}</p>
      </div>

      {/* Descrizione */}
      <p className="text-sm text-gray-300 max-w-xs leading-relaxed">
        {t.aboutDescription}
      </p>

      {/* Footer info */}
      <div className="text-xs text-gray-500 space-y-2">
        <p>{t.aboutLicense} · © 2025 zoott28354</p>
        <button
          onClick={() =>
            invoke("open_url", {
              url: "https://github.com/zoott28354/Youtube-Blocker",
            })
          }
          className="text-blue-400 hover:text-blue-300 transition-colors underline"
        >
          {t.aboutViewOnGithub}
        </button>
      </div>
    </div>
  );
}
