# Development & Setup Guide

This guide provides step-by-step instructions to set up the development environment for ZenYT on Windows, macOS, or Linux.

## 1. Prerequisites

Before you begin, ensure you have the following installed:

### A. Node.js & Package Manager
- Install **Node.js** (v18 or higher): [Download here](https://nodejs.org/)
- We recommend using `npm` (comes with Node.js) or `pnpm`.

### B. Rust & Build Tools
- Install **Rustup**: [Download here](https://rustup.rs/)
- **Windows**: You must install the "C++ build tools" for Visual Studio 2013 or later. The easiest way is to install [Build Tools for Visual Studio 2022](https://visualstudio.microsoft.com/visual-cpp-build-tools/). Make sure "Desktop development with C++" is selected.
- **macOS**: Run `xcode-select --install` in your terminal.
- **Linux**: Install build-essential, curl, wget, libssl-dev, libgtk-3-dev, libayatana-appindicator3-dev, librsvg2-dev.

### C. yt-dlp & FFmpeg
During development, having these in your system PATH makes testing easier:
- **yt-dlp**: Download the binary from [yt-dlp releases](https://github.com/yt-dlp/yt-dlp/releases) and add it to your PATH.
- **ffmpeg**: Required for merging video/audio formats. [Download FFmpeg](https://ffmpeg.org/download.html).

## 2. Project Initialization

If the repository does not have the `package.json` yet, you will need to initialize Tauri:
```bash
# To initialize the project (if starting from scratch)
npm create tauri-app@latest
# Follow the prompts: choose React, choose JavaScript/TypeScript, choose Vite.
```

If the code is already initialized:
```bash
git clone <repo-url>
cd ZenYT
npm install
```

## 3. Running in Development Mode

To start the Vite development server and the Tauri Rust window concurrently, run:
```bash
npm run tauri dev
```
*Note: The first time you run this, Cargo (Rust's package manager) will download and compile all backend dependencies. This may take a few minutes.*

## 4. Building for Production

When you are ready to create an installer (e.g., an `.msi` or `.exe` on Windows, a `.dmg` on macOS, or an `.AppImage` on Linux):
```bash
npm run tauri build
```
The compiled binaries and installers will be located in `src-tauri/target/release/bundle/`.

## 5. Troubleshooting
- **WebView2 Error (Windows)**: If the app opens as a blank white screen, ensure you have Microsoft Edge WebView2 runtime installed (comes pre-installed on Windows 11).
- **yt-dlp not found**: Ensure your Rust code is looking at the correct absolute path or that `yt-dlp` is properly exposed to the environment variables.