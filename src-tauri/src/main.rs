// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::import::{*};
use crate::data::{*};
mod import;
mod data;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            scan_local_logs,
            download_logs,
            parse_logs,
            guess_user_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
