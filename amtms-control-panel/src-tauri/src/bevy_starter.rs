use std::io::{BufRead, BufReader};
use std::process::Command;

#[tauri::command]
pub async fn start_bevy(app_handle: tauri::AppHandle) {
    let mut child = Command::new("rustProject")
        .args(&["--http", "127.0.0.1:7443"])
        .spawn()
        .expect("Failed to start Bevy process");
    // 读取子进程的标准输出
    loop {
        // 等待子进程结束
        if let Ok(status) = child.wait() {
            app_handle.exit(0)
        }
    }


}