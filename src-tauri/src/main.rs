// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
use commands::*;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Load the last viewed directory path when the application starts
            let config = Config::load();
            if let Some(root_path) = config.root_path {
                println!("Root path: {}", root_path);
                // You can add code here to use the root_path in your application
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            select_directory,
            create_init_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
