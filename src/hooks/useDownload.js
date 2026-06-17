import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export function useDownload() {
  const [progress, setProgress] = useState(null);
  const [downloading, setDownloading] = useState(false);

  useEffect(() => {
    let unlistenProgress;
    let unlistenComplete;

    async function setupListeners() {
      unlistenProgress = await listen("download-progress", (event) => {
        setProgress(event.payload);
      });

      unlistenComplete = await listen("download-complete", () => {
        setDownloading(false);
        setProgress(null);
        alert("Download Complete!");
      });
    }

    setupListeners();

    return () => {
      if (unlistenProgress) unlistenProgress();
      if (unlistenComplete) unlistenComplete();
    };
  }, []);

  const startDownload = async (url) => {
    setDownloading(true);
    setProgress({ percentage: "0", speed: "...", eta: "..." });
    try {
      await invoke("download_video", { url });
    } catch (e) {
      console.error(e);
      setDownloading(false);
    }
  };

  return { progress, downloading, startDownload };
}
