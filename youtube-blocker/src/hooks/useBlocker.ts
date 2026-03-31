import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface BlockStatus {
  hosts_blocked: boolean;
  firewall_active: boolean;
  browser_policy: boolean;
  block_doh_enabled: boolean;
  active_lists_count: number;
  active_list_names: string[];
}

export interface BlockList {
  id: string;
  name: string;
  sites: string[];
  active: boolean;
  builtin: boolean;
}

export function useBlocker() {
  const [status, setStatus] = useState<BlockStatus | null>(null);
  const [lists, setLists] = useState<BlockList[]>([]);
  const [hasPinSet, setHasPinSet] = useState<boolean | null>(null);
  const [isSessionUnlocked, setIsSessionUnlocked] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      const [s, listData, pinSet] = await Promise.all([
        invoke<BlockStatus>("get_status"),
        invoke<BlockList[]>("get_lists"),
        invoke<boolean>("has_pin"),
      ]);
      setStatus(s);
      setLists(listData);
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

  const createList = useCallback(
    async (name: string): Promise<BlockList> => {
      const list = await invoke<BlockList>("create_list", { name });
      await refresh();
      return list;
    },
    [refresh]
  );

  const updateList = useCallback(
    async (id: string, name: string, sites: string[]) => {
      await invoke("update_list", { id, name, sites });
      await refresh();
    },
    [refresh]
  );

  const deleteList = useCallback(
    async (id: string) => {
      await invoke("delete_list", { id });
      await refresh();
    },
    [refresh]
  );

  const toggleList = useCallback(
    async (id: string, active: boolean) => {
      await invoke("toggle_list", { id, active });
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

  const resetPin = useCallback(async () => {
    await invoke("reset_pin");
    setIsSessionUnlocked(false);
    await refresh();
  }, [refresh]);

  const sessionUnlock = useCallback(async (pin: string) => {
    // Lancia eccezione se PIN errato (PinModal mostra errore inline)
    await invoke("check_pin", { pin });
    setIsSessionUnlocked(true);
  }, []);

  return {
    status,
    lists,
    hasPinSet,
    isSessionUnlocked,
    loading,
    error,
    blockAll,
    unblockAll,
    createList,
    updateList,
    deleteList,
    toggleList,
    setPin,
    changePin,
    resetPin,
    sessionUnlock,
    refresh,
  };
}
