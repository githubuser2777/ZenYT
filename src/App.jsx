import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useDownload } from "./hooks/useDownload";
import "./App.css";

function App() {
  const [url, setUrl] = useState("");
  const [videoInfo, setVideoInfo] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  
  const { progress, downloading, downloadError, downloadComplete, startDownload } = useDownload();

  async function fetchInfo() {
    if (!url) return;
    setLoading(true);
    setError(null);
    setVideoInfo(null);
    try {
      const result = await invoke("get_video_info", { url });
      setVideoInfo(result);
    } catch (e) {
      setError(e);
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className="container">
      <h1>ZenYT</h1>
      <p>Minimal yt-dlp GUI</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          fetchInfo();
        }}
      >
        <input
          value={url}
          onChange={(e) => setUrl(e.currentTarget.value)}
          placeholder="Enter YouTube URL..."
          style={{ width: "300px" }}
          disabled={downloading}
        />
        <button type="submit" disabled={loading || downloading}>
          {loading ? "Fetching..." : "Fetch Info"}
        </button>
      </form>

      {error && <p style={{ color: "#ff6b6b", marginTop: "1rem" }}>Error: {error}</p>}

      {videoInfo && (
        <div className="info-card">
          {videoInfo.thumbnail && (
            <img src={videoInfo.thumbnail} alt="Thumbnail" />
          )}
          <h3>{videoInfo.title || "Unknown Title"}</h3>
          <p>
            Duration: {videoInfo.duration ? `${videoInfo.duration}s` : "Unknown"} | 
            Uploader: {videoInfo.uploader || "Unknown"}
          </p>
          
          <button 
            style={{ marginTop: "15px", width: "100%" }}
            onClick={() => startDownload(url)}
            disabled={downloading}
          >
            {downloading ? "Downloading..." : "Download Video"}
          </button>
          
          {downloadError && <p style={{ color: "#ff6b6b", marginTop: "1rem" }}>Download Failed: {downloadError}</p>}
          {downloadComplete && <p style={{ color: "#51cf66", marginTop: "1rem" }}>Download Complete!</p>}
          
          {progress && (
            <div style={{ marginTop: "15px" }}>
              <div style={{ background: "#333", height: "10px", borderRadius: "5px", overflow: "hidden" }}>
                <div style={{ background: "#24c8db", height: "100%", width: `${progress.percentage}%`, transition: "width 0.2s" }} />
              </div>
              <p style={{ marginTop: "5px", textAlign: "center" }}>
                {progress.percentage}% | Speed: {progress.speed} | ETA: {progress.eta}
              </p>
            </div>
          )}
        </div>
      )}
    </main>
  );
}

export default App;
