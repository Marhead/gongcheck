// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::{CustomMenuItem, Submenu, Menu};
mod commands;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[derive(serde::Deserialize)]
struct MyCommand {
    filename: String,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![

            commands::select_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
