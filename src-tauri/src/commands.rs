use std::env;
use std::path::PathBuf;

// TODO: Check for Mac
#[tauri::command]
pub fn select_directory_for_mac() -> Option<PathBuf> {
    let file_dialog = tauri::api::dialog::blocking::FileDialogBuilder::new()
        .set_directory("~")
        .pick_folder();

    file_dialog
}

#[tauri::command]
pub async fn select_directory(app_handle: tauri::AppHandle) -> Option<String> {
    let home_dir = env::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    tauri::api::dialog::blocking::FileDialogBuilder::new()
        .set_directory(home_dir)
        .pick_folder()
        .and_then(|path_buf| path_buf.to_str().map(String::from))
}