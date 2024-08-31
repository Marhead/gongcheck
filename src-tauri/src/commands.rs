use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::Path;
use dirs;
use serde_json::{json, to_string_pretty};
use tauri::api::dialog::blocking::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub root_path: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = Config::config_path();
        if config_path.exists() {
            let config_content = fs::read_to_string(config_path).unwrap_or_default();
            serde_json::from_str(&config_content).unwrap_or_default()
        } else {
            Config { root_path: None }
        }
    }

    fn save(&self) {
        let config_path = Config::config_path();
        let config_content = serde_json::to_string_pretty(self).unwrap();
        fs::write(config_path, config_content).unwrap();
    }

    fn config_path() -> std::path::PathBuf {
        let mut config_dir = dirs::config_dir().unwrap();
        config_dir.push("my_tauri_app");
        fs::create_dir_all(&config_dir).unwrap();
        config_dir.push("config.json");
        config_dir
    }
}

#[tauri::command]
pub async fn select_directory() -> Result<Option<String>, String> {
    // home_directory를 우선 불러온다.
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;

    // FileDialogBuilder를 이용하여 폴더 선택 다이얼로그를 띄운다.
    let selected_path = FileDialogBuilder::new()
        .set_directory(home_dir)
        .pick_folder()
        .and_then(|path_buf| path_buf.to_str().map(String::from)); // PathBuf를 String으로 변환한다.

    if let Some(ref path) = selected_path {
        let mut config = Config::load();
        config.root_path = Some(path.clone());
        config.save();
    }

    Ok(selected_path)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn create_init_file(folder_name: String) -> Result<String, String> {
    // string으로 받은 폴더를 Path로 변환한다.
    let folder_path = Path::new(&folder_name);

    // Ensure the directory exists or create it if it doesn't
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path).map_err(|e| e.to_string())?;
    }

    // Generate the folders
    let folders = vec![
        "character", "item", "location", "culture", "discovery", "note", "organization", "place", "relation", "specy", "story"
    ];

    for folder in folders {
        let folder_path = folder_path.join(folder);
        if !folder_path.exists() {
            fs::create_dir(&folder_path).map_err(|e| e.to_string())?;
        }
    }

    // 현재 시간 입력
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    // Construct the "config.json" file manually
    let save_path = folder_path.join(format!("config.json"));

    // Check if the file already exists using Rust's standard library
    if save_path.exists() {
        // Read the existing JSON content from the file
        let existing_content = fs::read_to_string(&save_path).map_err(|e| e.to_string())?;

        // Parse the existing JSON content into a serde_json::Value
        let mut json_value: serde_json::Value = serde_json::from_str(&existing_content).map_err(|e| e.to_string())?;

        // Update the "updated_at" field with the current time
        json_value["updated_at"] = json!(current_time);

        // Serialize the updated JSON content to a pretty string
        let updated_json_string = to_string_pretty(&json_value).map_err(|e| e.to_string())?;

        // Write the updated JSON string back to the file
        fs::write(&save_path, updated_json_string).map_err(|e| e.to_string())?;

        return Ok("Init file updated: ".to_string() + &save_path.to_string_lossy());
    }

    let json_content = json!({
        "name": folder_name,
        "created_at": current_time,
        "updated_at": current_time,
    });
    
    // Serialize the JSON content to a pretty string
    let json_string = to_string_pretty(&json_content).map_err(|e| e.to_string())?;

    // Write the serialized JSON string to the save path using Rust's standard library
    fs::write(&save_path, json_string).map_err(|e| e.to_string())?;

    Ok("Init file created: ".to_string() + &save_path.to_string_lossy())
}