use std::path::PathBuf;
use tauri::Manager;
use tauri::api::dialog;

#[tauri::command]
pub fn select_directory() -> Option<PathBuf> {
    let file_dialog = tauri::api::dialog::blocking::FileDialogBuilder::new()
        .set_directory("~")
        .pick_folder();

    match file_dialog {
        Ok(Some(path)) => Some(path),
        _ => None,
    }
}

// #[tauri::command]
// fn create_file(window: tauri::Window, command: MyCommand) -> Result<(), String> {
//     use std::fs::File;
//     match File::create(&command.filename) {
//         Ok(_) => Ok(()),
//         Err(e) => Err(e.to_string()),
//     }
// }