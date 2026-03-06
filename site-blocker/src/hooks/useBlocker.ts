import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface BlockStatus {
  hosts_blocked: boolean;
  firewall_active: boolean;
}

export function useBlocker() {
  const [status, setStatus] = useState<BlockStatus | null>(null);
  const [sites, setSites] = useState<string[]>([]);
  const [hasPinSet, setHasPinSet] = useState<boolean | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      const [s, siteList, pinSet] = await Promise.all([
        invoke<BlockStatus>("get_status"),
        invoke<string[]>("get_sites"),
        invoke<boolean>("has_pin"),
      ]);
      setStatus(s);
      setSites(siteList);
      setHasPinSet(pinSet);
      setError(null);
    } catch (e) {
      setError(String(e));
    }
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const blockAll = useCallback(async () => {
    setLoading(true);
    try {
      await invoke("block_all");
      await refresh();
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [refresh]);

  const unblockAll = useCallback(
    async (pin: string) => {
      setLoading(true);
      try {
        await invoke("unblock_all", { pin });
        await refresh();
      } finally {
        setLoading(false);
      }
      // Non cattura: rilancia al chiamante (PinModal mostra errore inline)
    },
    [refresh]
  );

  const addSite = useCallback(
    async (domain: string) => {
      await invoke("add_site", { domain });
      await refresh();
    },
    [refresh]
  );

  const removeSite = useCallback(
    async (domain: string) => {
      await invoke("remove_site", { domain });
      await refresh();
    },
    [refresh]
  );

  const setPin = useCallback(
    async (pin: string) => {
      await invoke("set_pin", { pin });
      await refresh();
    },
    [refresh]
  );

  const changePin = useCallback(
    async (oldPin: string, newPin: string) => {
      await invoke("change_pin", { oldPin, newPin });
      await refresh();
    },
    [refresh]
  );

  return {
    status,
    sites,
    hasPinSet,
    loading,
    error,
    blockAll,
    unblockAll,
    addSite,
    removeSite,
    setPin,
    changePin,
    refresh,
  };
}
