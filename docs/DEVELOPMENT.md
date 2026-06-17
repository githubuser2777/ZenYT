# Development Guide

## Prerequisites
- **Node.js**: >= 18.x
- **Rust**: Latest stable version
- **yt-dlp**: Needs to be available in the system PATH or downloaded to a specific directory for the app to use.
- **ffmpeg**: Required for yt-dlp.

## Setup Instructions

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/ZenYT.git
   cd ZenYT
   ```

2. **Install Frontend Dependencies**
   ```bash
   npm install
   ```

3. **Run in Development Mode**
   ```bash
   npm run tauri dev
   ```
   *This command will start the Vite dev server and the Tauri rust backend concurrently.*

## Building for Production
To build a stand-alone executable:
```bash
npm run tauri build
```
