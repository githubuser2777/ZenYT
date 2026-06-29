use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tauri::{AppHandle, Emitter, Manager};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

// Ponytail: Ưu tiên tìm yt-dlp và ffmpeg trong thư mục resource bundle. Nếu không có thì fallback dùng system PATH.
fn get_yt_dlp_path(app: &AppHandle) -> String {
    let exe_name = if cfg!(target_os = "windows") { "yt-dlp.exe" } else { "yt-dlp" };
    if let Ok(res_dir) = app.path().resource_dir() {
        let bundled = res_dir.join("bin").join(exe_name);
        if bundled.exists() {
            return bundled.to_string_lossy().to_string();
        }
    }
    "yt-dlp".to_string()
}

fn get_ffmpeg_args(app: &AppHandle) -> Vec<String> {
    let exe_name = if cfg!(target_os = "windows") { "ffmpeg.exe" } else { "ffmpeg" };
    if let Ok(res_dir) = app.path().resource_dir() {
        let bundled = res_dir.join("bin").join(exe_name);
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

    let mut cmd = Command::new(get_yt_dlp_path(&app));
    cmd.args(args);
    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

    let output = cmd.output()
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

#[derive(Clone, serde::Serialize)]
struct DownloadErrorPayload {
    error: String,
}

#[tauri::command]
pub fn download_video(app: AppHandle, url: String, output_dir: Option<String>, format: Option<String>) -> Result<(), String> {
    let yt_dlp_path = get_yt_dlp_path(&app);
    let ffmpeg_args = get_ffmpeg_args(&app);

    std::thread::spawn(move || {
        let mut args = vec!["--newline".to_string()];

        if let Some(dir) = output_dir {
            args.push("-P".to_string());
            args.push(dir);
        }

        match format.as_deref() {
            Some("audio") => {
                args.push("-x".to_string());
                args.push("--audio-format".to_string());
                args.push("mp3".to_string());
            }
            Some("1080p") => {
                args.push("-f".to_string());
                args.push("bv*[ext=mp4][height<=1080]+ba[ext=m4a]/b[ext=mp4]".to_string());
            }
            _ => {
                // best by default
            }
        }

        args.push(url);
        args.extend(ffmpeg_args);

        let mut cmd = Command::new(yt_dlp_path);
        cmd.args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
            
        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                let _ = app.emit("download-error", DownloadErrorPayload { error: format!("Failed to spawn yt-dlp: {}", e) });
                return;
            }
        };

        let stdout = match child.stdout.take() {
            Some(s) => s,
            None => {
                let _ = app.emit("download-error", DownloadErrorPayload { error: "Failed to capture stdout".to_string() });
                return;
            }
        };
        
        let stderr = match child.stderr.take() {
            Some(s) => s,
            None => {
                let _ = app.emit("download-error", DownloadErrorPayload { error: "Failed to capture stderr".to_string() });
                return;
            }
        };

        // Spawn a thread to concurrently read stderr and prevent pipe deadlocks
        let stderr_thread = std::thread::spawn(move || {
            use std::io::Read;
            let mut stderr_string = String::new();
            let mut reader = BufReader::new(stderr);
            let _ = reader.read_to_string(&mut stderr_string);
            stderr_string
        });

        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("[download]") {
                    let mut percent = String::new();
                    let mut speed = String::new();
                    let mut eta = String::new();

                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for i in 1..parts.len() {
                        if parts[i].ends_with('%') {
                            percent = parts[i].trim_end_matches('%').to_string();
                        } else if parts[i] == "at" && i + 1 < parts.len() {
                            speed = parts[i + 1].to_string();
                        } else if parts[i] == "ETA" && i + 1 < parts.len() {
                            eta = parts[i + 1].to_string();
                        }
                    }

                    if !percent.is_empty() {
                        let _ = app.emit("download-progress", ProgressPayload {
                            percentage: percent,
                            speed,
                            eta
                        });
                    }
                }
            }
        }
        
        let stderr_output = stderr_thread.join().unwrap_or_default();

        match child.wait() {
            Ok(status) => {
                if status.success() {
                    let _ = app.emit("download-complete", ());
                } else {
                    let _ = app.emit("download-error", DownloadErrorPayload { 
                        error: format!("yt-dlp failed: {}", stderr_output) 
                    });
                }
            }
            Err(e) => {
                let _ = app.emit("download-error", DownloadErrorPayload { 
                    error: format!("Failed to wait for yt-dlp: {}", e) 
                });
            }
        }
    });
    
    Ok(())
}
