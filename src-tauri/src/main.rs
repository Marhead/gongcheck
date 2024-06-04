// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WindowBuilder, WindowUrl, CustomMenuItem, Submenu, Menu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let new_file = CustomMenuItem::new("new".to_string(), "New");
    let open_file = CustomMenuItem::new("open".to_string(), "Open");
    let save_file = CustomMenuItem::new("save".to_string(), "Save");

    let file_menu = Submenu::new("File", Menu::new()
        .add_item(new_file)
        .add_item(open_file)
        .add_item(save_file));

    let edit_undo = CustomMenuItem::new("undo".to_string(), "Undo");
    let edit_redo = CustomMenuItem::new("redo".to_string(), "Redo");

    let edit_menu = Submenu::new("Edit", Menu::new()
        .add_item(edit_undo)
        .add_item(edit_redo));

    let menu = Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
        .add_item(quit);

    tauri::Builder::default()
        .setup(|app| {
            let window = WindowBuilder::new(app, "main", WindowUrl::default())
                .title("Gongcheck")
                .decorations(false)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
