use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};
use std::path::Path;
use dirs;
use serde_json::{json, to_string_pretty};
use tauri::api::dialog::blocking::*;
use serde::Serialize;

#[derive(Serialize)]
struct CreateInitFileParams {
    folder_name: String,
}

#[tauri::command]
pub async fn select_directory() -> Result<Option<String>, String> {
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let selected_path = FileDialogBuilder::new()
        .set_directory(home_dir)
        .pick_folder()
        .and_then(|path_buf| path_buf.to_str().map(String::from));

    Ok(selected_path)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn create_init_file(folder_name: String) -> Result<String, String> {
    // let home_dir = dirs::home_dir().ok_or_else(|| "Unable to get home directory".to_string())?;

    let folder_path = Path::new(&folder_name);

    // Ensure the directory exists or create it if it doesn't
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path).map_err(|e| e.to_string())?;
    }

    // Construct the save path manually
    let save_path = folder_path.join(format!("{}.json", folder_name));

    // Check if the file already exists using Rust's standard library
    if save_path.exists() {
        return Err("Init file already exists".to_string());
    }

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let json_content = json!({
        "name": folder_name,
        "created_at": current_time,
    });
    
    // Serialize the JSON content to a pretty string
    let json_string = to_string_pretty(&json_content).map_err(|e| e.to_string())?;

    // Write the serialized JSON string to the save path using Rust's standard library
    fs::write(&save_path, json_string).map_err(|e| e.to_string())?;

    Ok("Init file created".to_string())
}