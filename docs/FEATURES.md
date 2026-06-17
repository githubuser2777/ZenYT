# Comprehensive Feature Set

ZenYT is designed to expose the immense power of `yt-dlp` through an intuitive GUI. Here is the detailed breakdown of features and how they map to `yt-dlp` commands.

## 1. Video Parsing & Analysis
**User Flow:** The user pastes a URL into the input field.
**Backend Action:** `yt-dlp --dump-json <URL>`
**UI Output:** 
- Displays the video title, thumbnail, duration, and channel name.
- Displays a dropdown of available formats (e.g., Video: 1080p MP4, Audio: 128kbps M4A).

## 2. Download Modalities
**Standard Video Download (Best Quality):**
- Command: `yt-dlp -f "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best" <URL>`
**Audio Only Extraction:**
- Command: `yt-dlp -x --audio-format mp3 --audio-quality 0 <URL>`
- UI: A simple toggle switch for "Audio Only".

## 3. Playlists & Channels
- The UI will detect if the URL contains `list=`.
- It will offer an option: "Download entire playlist" vs "Download single video".
- Command: `yt-dlp --yes-playlist <URL>` vs `yt-dlp --no-playlist <URL>`.

## 4. Subtitles & Metadata
- **Subtitles**: Users can check a box to download subs.
  - Command: `yt-dlp --write-sub --write-auto-sub --sub-lang en,vi`
- **Metadata**: Users can choose to embed the thumbnail and metadata.
  - Command: `yt-dlp --embed-thumbnail --embed-metadata`

## 5. Progress Monitoring
- The core feature of the UI. `yt-dlp` naturally outputs text like:
  `[download]  45.5% of 50.00MiB at 2.50MiB/s ETA 00:15`
- The Rust backend uses Regex to parse this string in real-time, extracting:
  - Percentage: `45.5`
  - Total Size: `50.00MiB`
  - Speed: `2.50MiB/s`
  - ETA: `00:15`
- The UI animates a progress bar based on these values.

## 6. Power User Settings
- A hidden/advanced tab where users can type raw CLI arguments.
- E.g., `--proxy socks5://...` or `--cookies-from-browser chrome`.