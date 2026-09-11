<div align="center">

# 🌌 ZenYT

**The Ultimate Lightweight YouTube Desktop Client**

[![License: GPL-3.0-only](https://img.shields.io/badge/License-GPL--3.0--only-blue.svg)](LICENSE)
[![Tauri Version](https://img.shields.io/badge/Tauri-v2.0-blue.svg?logo=tauri)](https://tauri.app)
[![React Version](https://img.shields.io/badge/React-v19.0-61dafb.svg?logo=react)](https://react.dev)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Platform Support](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](#-platform-support)

ZenYT is a next-generation, high-performance desktop application that wraps the raw power of `yt-dlp` in a gorgeous, modern, and hardware-accelerated Graphical User Interface (GUI).

</div>

---

## 🌟 Introduction

Unlike heavy, memory-hungry Electron applications, ZenYT is built using **Tauri (Rust)** and **React 19**, rendering via the OS's native webview. This results in near-instant startup times, minimal CPU usage, and an installer size under 10MB. 

ZenYT is designed around **minimalism, premium aesthetics, and speed**. It employs a custom glassmorphic Dark Mode design tailored to minimize eye strain and maximize functional clarity.

---

## ✨ Key Features

- 🚀 **Real-Time Parsing & Analysis**: Fetches complete media metadata, thumbnails, uploaders, and available formats asynchronously using `yt-dlp --dump-json`.
- 📊 **Interactive Progress HUD**: Beautifully renders download progress, speed, ETA, and size by parsing `yt-dlp` output line-by-line via the Rust backend.
- 🛠️ **Self-Contained Binary Fallback**: Automatically looks for bundled `yt-dlp` and `ffmpeg` binaries inside the application resources before falling back to system PATH.
- 🎨 **Glassmorphism Design System**: Strictly hand-crafted Vanilla CSS with CSS variables, smooth animations, and interactive hover feedback. No bloated styling frameworks.
- ⚡ **Resource Efficient**: Operates on a fraction of the memory footprint of traditional desktop wrappers.

---

## 🏗️ Architecture

ZenYT utilizes Tauri's secure Inter-Process Communication (IPC) bridge:

```mermaid
graph TD
    %% Styling
    classDef React fill:#20232a,stroke:#61dafb,stroke-width:2px,color:#61dafb;
    classDef Rust fill:#2b2b2b,stroke:#e05d44,stroke-width:2px,color:#e05d44;
    classDef External fill:#1f232a,stroke:#34d058,stroke-width:2px,color:#34d058;
    classDef OS fill:#0d1117,stroke:#58a6ff,stroke-width:1px,color:#c9d1d9;

    %% Nodes
    UI[React 19 Frontend Webview]:::React
    Core[Tauri Rust Core]:::Rust
    YTDLP[yt-dlp CLI Process]:::External
    FFMPEG[ffmpeg CLI Process]:::External
    FS[Local File System]:::OS

    %% Connections
    UI <-->|Tauri IPC Events & Commands| Core
    Core -->|Spawn Child Process| YTDLP
    Core -->|Spawn Helper Process| FFMPEG
    YTDLP -->|Buffered stdout/stderr| Core
    Core -->|Write Media Files| FS
```

### Electron vs. Tauri Comparison

| Feature | Electron (Typical) | Tauri (ZenYT) |
| :--- | :--- | :--- |
| **Idle Memory Usage** | ~300 MB - 500 MB | **~30 MB - 50 MB** |
| **Installer Size** | ~100 MB+ | **~5 MB - 10 MB** |
| **Backend Language** | Node.js (JavaScript) | **Rust (Memory Safe, Fast)** |
| **UI Engine** | Bundled Chromium | **Native Webview (WebView2/WebKit)** |

---

## 📂 Project Structure

```text
ZenYT/
├── src-tauri/               # Rust Backend (Tauri Host)
│   ├── Cargo.toml           # Backend dependencies
│   └── src/
│       ├── main.rs          # App entry point
│       ├── lib.rs           # Tauri builder & commands
│       └── commands.rs      # Process spawning & IPC
├── src/                     # React Frontend (Webview Client)
│   ├── hooks/               # Custom React hooks
│   ├── App.jsx              # Main UI component
│   ├── App.css              # Global styling & design system
│   └── main.jsx             # React DOM renderer
├── docs/                    # Technical Documentation
├── package.json             # Frontend dependencies
└── vite.config.js           # Vite build configuration
```

---

## 🛠️ Getting Started

### Prerequisites
1. **Node.js**: v18+ is recommended.
2. **Rust**: Cargo & Rustup compiler suite.
3. **C++ Build Tools** (Windows): Install Visual Studio Build Tools with "Desktop development with C++".

Detailed system-specific setup guides can be found in [DEVELOPMENT.md](docs/DEVELOPMENT.md).

### Installation & Execution

Clone the repository and install the dependencies:
```bash
git clone https://github.com/your-org/ZenYT.git
cd ZenYT
npm install
```

Start the Vite dev server and Tauri window concurrently:
```bash
npm run tauri dev
```

### Production Build

To build a production-ready package/installer for your native platform:
```bash
npm run tauri build
```
The output installer (e.g. `.msi`, `.dmg` or `.deb`) will be generated inside `src-tauri/target/release/bundle/`.

---

## 📚 Technical Documentation

Explore the following markdown files inside the `docs/` folder to understand more:
* 📖 [Architecture Blueprint](docs/ARCHITECTURE.md) — Learn about Tauri commands, Rust event emitting, and CLI bindings.
* ⚙️ [Development Setup](docs/DEVELOPMENT.md) — Step-by-step setup, dependencies installation, and troubleshooting.
* 📋 [Feature Specifications](docs/FEATURES.md) — Mapping of UI functionalities to `yt-dlp` commands.
* 🎨 [UI & UX Guidelines](docs/UI_UX_GUIDELINES.md) — Theme tokens, transition animations, and color system.

---

## 🤝 Contributing

We welcome contributions! Please review our [Code of Conduct](CODE_OF_CONDUCT.md) and [Contributing Guidelines](CONTRIBUTING.md) before submitting a Pull Request.

- Ensure no external CSS libraries (Tailwind, Bootstrap) are introduced. **Keep it Vanilla!**
- Run `cargo check` and verify that the app launches without errors.

---

## 📄 License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0-only). See the [LICENSE](LICENSE) file for details.

<div align="center">
  <br />
  <p><i>Built with ☕ and Rust</i></p>
</div>