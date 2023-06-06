#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

use std::path::Path;

mod helper;
mod download;
mod env;
mod launcher;
mod window;
mod log;

#[tokio::main]
async fn main() {
    let path = Path::new(env::PATH);
    let filename = path
        .file_name()
        .expect("failed to get file name")
        .to_string_lossy()
        .into_owned();
    log!("INFO", filename, "Checking for updates...");
    let window_info = helper::WindowInfo::new(0..100);
    'dl_file: {
        if path.exists() {
            log!("OK", filename, "File exists! Skip download!");
            break 'dl_file;
        }
        log!("INFO", filename, "Downloading...");
        match download::download_file(env::URL, env::PATH, |delta: u32| -> bool {
            if let Err(_) = window_info.update(window::Signal::AdvanceDelta(delta)) {
                log!("WARN", "Window closed!");
                return true;
            }
            false
        }).await {
            Ok(_) => log!("OK", filename, "Downloaded!"),
            Err(download::DownloadError::DownloadStopped) => {
                log!("ERROR", filename, "Download stopped");
                if let Err(e) = std::fs::remove_file(path) {
                    log!("ERROR", filename, "while removing file: {}", e);
                }
                return;
            }
            Err(e) => {
                log!("ERROR", filename, "while downloading: {}", e);
                return;
            }
        }
    }
    window_info.stop();
    log!("INFO", "Executing launcher!");
    if let Some(launcher) = &*env::LAUNCHER {
        match launcher.execute() {
            Ok(_) => log!("OK", "Launcher executed!"),
            Err(e) => log!("ERROR", "Error while executing launcher: {}", e),
        }
    }
}