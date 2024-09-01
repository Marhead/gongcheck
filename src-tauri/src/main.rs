// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
use commands::*;
use serde_json::Value;
use std::fs;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Define the path to the config file in the home directory
            let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
            println!("Home directory: {:?}", home_dir);
            let gongcheck_dir = home_dir.join("Gongcheck");
            let settings_path = gongcheck_dir.join("settings.json");

            // Read the config file if it exists
            let settings: Value = if settings_path.exists() {
                let settings_content = fs::read_to_string(&settings_path).unwrap_or_default();
                serde_json::from_str(&settings_content).unwrap_or_default()
            } else {
                serde_json::json!({})
            };

            // Determine the initial route based on the config
            if let Some(last_project_path) = settings.get("last_project_path").and_then(|v| v.as_str()) {
                println!("Last project path: {}", last_project_path);
                // Emit an event to set the initial route to Overview with the last_project_path
                app.emit_all("set_initial_route", ("overview", last_project_path)).unwrap();
            } else {
                // Emit an event to set the initial route to Welcome
                app.emit_all("set_initial_route", "welcome").unwrap();
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
