# Architecture

ZenYT is a stand-alone desktop application built to be as lightweight and performant as possible.

## Tauri Architecture
1. **Core (Rust)**
   - Acts as the main application orchestrator.
   - Responsible for file system operations, window management, and native OS APIs.
   - Manages the execution of `yt-dlp` via `std::process::Command` (child processes).
   - Relays progress and metadata back to the UI using Tauri Events.

2. **UI (React + Vite)**
   - Rendered using the OS-native webview (Edge WebView2 on Windows, WebKit on macOS/Linux).
   - Communicates with the Rust core via `@tauri-apps/api`.
   - Styled with Vanilla CSS to achieve a premium aesthetic without the bloat of CSS frameworks.

3. **External Dependencies**
   - `yt-dlp`: The core downloading engine.
   - `ffmpeg`: Used by yt-dlp for media merging and conversion.
