import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export function useDownload() {
  const [progress, setProgress] = useState(null);
  const [downloading, setDownloading] = useState(false);
  const [downloadError, setDownloadError] = useState(null);
  const [downloadComplete, setDownloadComplete] = useState(false);

  useEffect(() => {
    const p1 = listen("download-progress", (event) => {
      setProgress(event.payload);
    });

    const p2 = listen("download-complete", () => {
      setDownloading(false);
      setProgress(null);
      setDownloadComplete(true);
      setTimeout(() => setDownloadComplete(false), 3000);
    });

    const p3 = listen("download-error", (event) => {
      setDownloading(false);
      setProgress(null);
      setDownloadError(event.payload.error);
      setTimeout(() => setDownloadError(null), 5000);
    });

    return () => {
      p1.then(fn => fn());
      p2.then(fn => fn());
      p3.then(fn => fn());
    };
  }, []);

  const startDownload = async (url) => {
    setDownloading(true);
    setDownloadError(null);
    setDownloadComplete(false);
    setProgress({ percentage: "0", speed: "...", eta: "..." });
    try {
      await invoke("download_video", { url });
    } catch (e) {
      console.error(e);
      setDownloading(false);
      setDownloadError(String(e));
    }
  };

  return { progress, downloading, downloadError, downloadComplete, startDownload };
}
