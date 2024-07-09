use std::path::PathBuf;

// TODO: Check for Mac
// #[tauri::command]
// pub fn select_directory() -> Option<PathBuf> {
//     let file_dialog = tauri::api::dialog::blocking::FileDialogBuilder::new()
//         .set_directory("~")
//         .pick_folder();

//     file_dialog
// }

#[tauri::command]
pub fn select_directory() -> Option<PathBuf> {
    let dialog = tauri::api::dialog::blocking::FileDialogBuilder::new()
        .set_directory("/")
        .pick_folder();
    
    dialog
}

// #[tauri::command]
// fn create_file(window: tauri::Window, command: MyCommand) -> Result<(), String> {
//     use std::fs::File;
//     match File::create(&command.filename) {
//         Ok(_) => Ok(()),
//         Err(e) => Err(e.to_string()),
//     }
// }