use std::path::PathBuf;
use dirs;

#[tauri::command]
pub async fn select_directory() -> Option<String> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    tauri::api::dialog::blocking::FileDialogBuilder::new()
        .set_directory(home_dir)
        .pick_folder()
        .and_then(|path_buf| path_buf.to_str().map(String::from))
}

#[tauri::command]
pub async fn create_init_file(folder_name: String) -> Result<(), String> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    let init_file = home_dir.join(format!("{}.json", folder_name));
    if init_file.exists() {
        return Err("Init file already exists".to_string());
    }
    std::fs::write(init_file, "Hello, Tauri!").map_err(|e| e.to_string())
}