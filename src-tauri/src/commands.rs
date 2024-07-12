use std::env;
use std::path::PathBuf;

#[tauri::command]
pub async fn select_directory(app_handle: tauri::AppHandle) -> Option<String> {
    let home_dir = env::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    tauri::api::dialog::blocking::FileDialogBuilder::new()
        .set_directory(home_dir)
        .pick_folder()
        .and_then(|path_buf| path_buf.to_str().map(String::from))
}