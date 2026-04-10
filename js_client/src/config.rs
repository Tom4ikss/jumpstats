use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use rfd::{FileDialog, MessageDialog, MessageLevel};


fn get_data_dir() -> Result<PathBuf, String> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "CS2", "JumpTracker") {
        let dir = proj_dirs.data_dir();
        if !dir.exists() {
            fs::create_dir_all(dir).map_err(|e| format!("Error while creating dir: {}", e))?;
        }
        Ok(dir.to_path_buf())
    } else {
        Err("Failed to find AppData dir".to_string())
    }
}

pub fn get_or_ask_log_path() -> PathBuf {
    let data_dir = get_data_dir().expect("Fatal error: no access to fs");
    let path_file = data_dir.join("log_path");

    // 1. Проверяем, сохраняли ли мы путь ранее
    if path_file.exists() {
        if let Ok(saved_path) = fs::read_to_string(&path_file) {
            let path = PathBuf::from(saved_path.trim());
            if path.exists() {
                println!("Found path to console.log!");
                return path;
            }
        }
    }
    
    MessageDialog::new()
        .set_title("CS2 Jump Tracker configuration")
        .set_description("You need to provide path to console.log file.\n\nE.g. \nC:/ProgramFiles(x86)/Steam/steamapps/common/Counter-Strike Global Offensive/game/csgo/console.log")
        .set_level(MessageLevel::Info)
        .show();
    
    let chosen_path = FileDialog::new()
        .set_title("Choose console.log file path")
        .add_filter("Log file", &["log"])
        .pick_file();

    match chosen_path {
        Some(path) => {
            fs::write(&path_file, path.to_string_lossy().to_string())
                .expect("Failed to save log file path");
            println!("Path saved!");
            path
        }
        None => {
            MessageDialog::new()
                .set_title("Error")
                .set_description("You need to provide path to console.log file")
                .set_level(MessageLevel::Error)
                .show();
            std::process::exit(1);
        }
    }
}