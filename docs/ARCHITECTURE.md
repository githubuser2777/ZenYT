# System Architecture

ZenYT follows a strict two-tier architecture enforced by the Tauri framework. This document explains how the frontend (React) communicates with the backend (Rust) and how `yt-dlp` is orchestrated.

## 1. High-Level Architecture

```mermaid
graph TD
    UI[React Frontend (WebView)] <-->|Tauri IPC (Events & Commands)| Core[Rust Backend]
    Core -->|Spawn Child Process| YTDLP[yt-dlp CLI]
    Core -->|Spawn Child Process| FFMPEG[ffmpeg CLI]
    YTDLP -->|stdout/stderr| Core
    Core -->|File System| Disk[Local Storage]
```

## 2. The Rust Backend (Core)
The Rust core acts as the secure bridge between the OS and the web frontend.

### 2.1 Process Orchestration
- We do not bundle `yt-dlp` directly into the binary to avoid bloating and license complications. Instead, the app checks if `yt-dlp` exists in the system PATH or downloads a portable version to the app's local data directory on first launch.
- Rust uses `std::process::Command` to spawn `yt-dlp`. 
- **Fetching Metadata**: When a user pastes a URL, Rust runs `yt-dlp -j <URL>`. It parses the JSON output to extract video title, thumbnail, duration, and available formats, then sends this struct to the React frontend.
- **Downloading**: Rust runs `yt-dlp` with the user's selected arguments. It captures `stdout` line by line.

### 2.2 IPC & Event Streaming
Downloading a file takes time. We cannot use a simple synchronous request-response.
- Rust uses `tauri::Window::emit` to send real-time events to the frontend.
- Event payload structure:
  ```json
  {
    "id": "download_task_123",
    "status": "downloading",
    "percentage": 45.5,
    "speed": "2.5MiB/s",
    "eta": "00:15"
  }
  ```

## 3. The React Frontend (UI)
- The UI is entirely state-driven. We use React Hooks to manage the download queue.
- **Listener Hooks**: A custom hook `useDownloadListener` subscribes to Tauri events using `listen('download-progress', callback)`.
- **Vanilla CSS**: We strictly use Vanilla CSS with CSS Modules or standard imports. No CSS-in-JS or heavy frameworks to ensure the WebView renders at 60fps effortlessly.

## 4. Folder Structure Projection
```text
ZenYT/
├── src-tauri/             # Rust Backend Code
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs        # Tauri entry point
│   │   ├── commands.rs    # IPC Commands callable from UI
│   │   └── downloader.rs  # yt-dlp wrapper logic
├── src/                   # React Frontend Code
│   ├── assets/            # Images, icons
│   ├── components/        # Reusable UI parts (Buttons, ProgressBars)
│   ├── hooks/             # Custom React hooks
│   ├── styles/            # Global Vanilla CSS variables
│   ├── App.jsx            # Main React component
│   └── main.jsx           # React entry point
└── package.json
```