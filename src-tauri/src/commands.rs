use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tauri::{AppHandle, Emitter};
use regex::Regex;

// Ponytail: Return raw Value instead of defining a massive struct. Let frontend pick what it needs.
#[tauri::command]
pub fn get_video_info(url: &str) -> Result<serde_json::Value, String> {
    let output = Command::new("yt-dlp")
        .args(["--dump-json", url])
        .output()
        .map_err(|e| format!("Failed to execute yt-dlp: {}", e))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp error: {}", err));
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(parsed)
}

#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
    percentage: String,
    speed: String,
    eta: String,
}

#[tauri::command]
pub fn download_video(app: AppHandle, url: String) -> Result<(), String> {
    std::thread::spawn(move || {
        let mut child = Command::new("yt-dlp")
            // --newline is crucial for parsing stdout line by line
            .args(["--newline", &url])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn yt-dlp");

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        
        // Example output: [download]  45.5% of 50.00MiB at 2.50MiB/s ETA 00:15
        let re = Regex::new(r"\[download\]\s+(?P<percent>[0-9\.]+)%.*at\s+(?P<speed>[a-zA-Z0-9\./~]+)\s+ETA\s+(?P<eta>[0-9:]+)").unwrap();

        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some(caps) = re.captures(&line) {
                    let percent = caps.name("percent").map_or("", |m| m.as_str()).to_string();
                    let speed = caps.name("speed").map_or("", |m| m.as_str()).to_string();
                    let eta = caps.name("eta").map_or("", |m| m.as_str()).to_string();
                    
                    let _ = app.emit("download-progress", ProgressPayload {
                        percentage: percent,
                        speed,
                        eta
                    });
                }
            }
        }
        let _ = child.wait();
        let _ = app.emit("download-complete", ());
    });
    
    Ok(())
}
