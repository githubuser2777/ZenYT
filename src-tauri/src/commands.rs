use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tauri::{AppHandle, Emitter, Manager};
use regex::Regex;

// Ponytail: Ưu tiên tìm yt-dlp và ffmpeg trong thư mục resource bundle. Nếu không có thì fallback dùng system PATH.
fn get_yt_dlp_path(app: &AppHandle) -> String {
    if let Ok(res_dir) = app.path().resource_dir() {
        let bundled = res_dir.join("bin").join("yt-dlp.exe");
        if bundled.exists() {
            return bundled.to_string_lossy().to_string();
        }
    }
    "yt-dlp".to_string()
}

fn get_ffmpeg_args(app: &AppHandle) -> Vec<String> {
    if let Ok(res_dir) = app.path().resource_dir() {
        let bundled = res_dir.join("bin").join("ffmpeg.exe");
        if bundled.exists() {
            return vec!["--ffmpeg-location".to_string(), bundled.to_string_lossy().to_string()];
        }
    }
    vec![]
}

#[tauri::command]
pub fn get_video_info(app: AppHandle, url: &str) -> Result<serde_json::Value, String> {
    let mut args = vec!["--dump-json".to_string(), url.to_string()];
    args.extend(get_ffmpeg_args(&app));

    let output = Command::new(get_yt_dlp_path(&app))
        .args(args)
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
    let yt_dlp_path = get_yt_dlp_path(&app);
    let ffmpeg_args = get_ffmpeg_args(&app);

    std::thread::spawn(move || {
        let mut args = vec!["--newline".to_string(), url];
        args.extend(ffmpeg_args);

        let mut child = Command::new(yt_dlp_path)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn yt-dlp");

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        
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
