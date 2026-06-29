import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useDownload } from "./hooks/useDownload";
import "./App.css";

function App() {
  const [url, setUrl] = useState("");
  const [videoInfo, setVideoInfo] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [format, setFormat] = useState("best");
  
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

  async function handleDownload() {
    try {
      const selectedPath = await open({
        directory: true,
        multiple: false,
        title: "Select Save Folder",
      });
      
      if (selectedPath) {
        startDownload(url, selectedPath, format);
      }
    } catch (err) {
      console.error("Failed to open dialog", err);
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
          <select 
            value={format} 
            onChange={(e) => setFormat(e.target.value)}
            disabled={downloading}
            style={{ marginTop: "15px", width: "100%", padding: "8px", borderRadius: "8px", border: "1px solid #444", background: "#2a2a2a", color: "#fff" }}
          >
            <option value="best">Best Quality (Video + Audio)</option>
            <option value="1080p">1080p MP4 (Video + Audio)</option>
            <option value="audio">Audio Only (MP3)</option>
          </select>
          
          <button 
            style={{ marginTop: "10px", width: "100%" }}
            onClick={handleDownload}
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
