#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod bevy_starter;
use bevy_starter::start_bevy;
fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![start_bevy])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
