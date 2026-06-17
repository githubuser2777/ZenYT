# ZenYT: The Ultimate Lightweight Video Downloader

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)
![Tech Stack](https://img.shields.io/badge/tech-Tauri%20%7C%20Rust%20%7C%20React-orange)

ZenYT is a next-generation, stand-alone desktop application designed to provide a stunning, premium Graphical User Interface (GUI) for the powerful command-line tool `yt-dlp`. 

Instead of relying on heavy frameworks like Electron, ZenYT is built from the ground up using **Tauri** (Rust) and **React**, ensuring minimal memory usage, a tiny binary size, and lightning-fast performance, while delivering a beautifully crafted, modern user experience.

## 🌟 Key Features

* **Complete `yt-dlp` Integration**: Access the full power of yt-dlp without opening a terminal.
* **Smart Format Selection**: Automatically fetch available formats (4K, 1080p, 60fps) and let the user choose, or auto-pick the best quality.
* **Audio Extraction**: Easily download and convert videos to MP3, FLAC, M4A, or WAV with high-quality audio encoders.
* **Playlist & Channel Support**: Paste a playlist or channel URL and batch download multiple videos simultaneously.
* **Real-time Progress Tracking**: Beautiful progress bars showing download speed, ETA, and file size.
* **Queue Management**: Pause, resume, cancel, and manage your download history seamlessly.
* **Metadata & Extras**: Download subtitles (VTT/SRT), thumbnails, and embed metadata directly into the media files.
* **Premium Dark Mode UI**: Crafted with Vanilla CSS for a smooth, glassmorphism-inspired aesthetic.

## 🚀 Why Tauri?

Unlike Electron apps which bundle an entire Chromium browser (often consuming 300MB+ RAM just idling), Tauri uses the OS's native webview (WebView2 on Windows, WebKit on macOS/Linux). 
- **Lightweight**: The application installer is typically under 10MB.
- **Fast**: Rust backend ensures safe, concurrent, and highly optimized execution.
- **Secure**: Strict communication channels between the UI and the system.

## 📚 Documentation Directory

To understand the project deeply, please refer to the extensive documentation in our `docs/` folder:
- **[DEVELOPMENT.md](docs/DEVELOPMENT.md)**: Setup your local environment, install dependencies, and build the app.
- **[ARCHITECTURE.md](docs/ARCHITECTURE.md)**: Understand the Inter-Process Communication (IPC) between Rust and React.
- **[FEATURES.md](docs/FEATURES.md)**: Detailed breakdown of yt-dlp capabilities mapped to the UI.
- **[UI_UX_GUIDELINES.md](docs/UI_UX_GUIDELINES.md)**: The design system, color palettes, and CSS rules.
- **[AGENTS.md](docs/AGENTS.md)**: Rules and context for AI coding assistants.

## 🤝 Contributing
We welcome contributions! Please read our [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details on our code of conduct, and the process for submitting pull requests to us.

## 📄 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.