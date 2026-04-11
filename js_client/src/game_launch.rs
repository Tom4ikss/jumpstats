use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use rfd::{MessageButtons, MessageDialog, MessageLevel};
use rfd::MessageDialogResult::{No, Yes};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};
use tokio::io::{BufReader, SeekFrom, AsyncSeekExt};
use tokio::fs;
use tokio::fs::File;
use winreg::enums::*;
use winreg::RegKey;


pub async fn attach_to_log(log_file_path: &PathBuf) -> anyhow::Result<BufReader<File>> {
    let file = loop {
        match File::open(log_file_path).await {
            Ok(f) => break f,
            Err(_) => {
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    };

    let mut reader = BufReader::new(file);

    reader.seek(SeekFrom::End(0)).await?;

    println!("Successfully connected to console log file.");
    Ok(reader)
}

pub async fn clear_log_file(path: &PathBuf) {
    match File::create(path).await {
        Ok(_) => println!("Log file cleared."),
        Err(e) => eprintln!("Error while clearing log file: {}", e),
    }
}

pub fn is_cs2_running(sys: &mut System) -> bool {
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing()
    );

    sys.processes().values().any(|p| {
        if let Some(name) = p.name().to_str() {
            name.eq_ignore_ascii_case("cs2.exe")
        } else {
            false
        }
    })
}

pub async fn wait_for_cs2_process(sys: &mut System) {
    if !is_cs2_running(sys) {
        println!("Waiting for CS2 launch...");
        loop {
            tokio::time::sleep(Duration::from_secs(3)).await;

            if is_cs2_running(sys) {
                break;
            }
        }
    }
}

pub async fn wait_for_engine_init(log_path: &PathBuf) {
    println!("Waiting for Source 2 fs init...");

    let mut attempts = 0;
    loop {
        match fs::metadata(log_path).await {
            Ok(metadata) => {
                if metadata.len() > 0 {
                    println!("CS2 starts to write to log file");
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    break;
                }
            }
            Err(_) => {
            }
        }

        tokio::time::sleep(Duration::from_millis(500)).await;

        attempts += 1;
        if attempts > 100 {
            println!("Out of attempts while waiting for CS2 starting to write to log file...");
            break;
        }
    }
}



pub async fn ensure_running_or_launch_cs2(sys: &mut System, log_path: &PathBuf) -> bool {
    if is_cs2_running(sys) {
        MessageDialog::new()
            .set_title("CS2 is already running")
            .set_description("Ensure that you launched cs2 with \"-condebug\" param in Steam. If no either restart it with this param yourself or use built in app game launch functionality.")
            .set_level(MessageLevel::Info)
            .show();
        true
    } else {
        let r = MessageDialog::new()
            .set_title("Launch CS2?")
            .set_description("You want to automatically launch CS2 with needed params?")
            .set_buttons(MessageButtons::YesNo)
            .set_level(MessageLevel::Info)
            .show();

        match r {
            Yes => {

                clear_log_file(&log_path).await;

                println!("Launching CS2...");
                if let Err(e) = launch_cs2_reliable() {
                    eprintln!("Error: {}", e);
                };
            }
            No => {
                println!("Waiting for manually cs2 launch...");
            }
            _ => {},
        }
        false
    }
}

pub fn launch_cs2_reliable() -> Result<(), String> {

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let steam_path = hkcu
        .open_subkey(r"Software\Valve\Steam")
        .and_then(|key| key.get_value::<String, _>("SteamExe"))
        .unwrap_or_else(|_| "C:/Program Files (x86)/Steam/steam.exe".to_string());

    println!("Steam found: {}", steam_path);

    match Command::new(steam_path)
        .args(["-applaunch", "730", "-condebug"])
        .spawn()
    {
        Ok(_) => {
            println!("Launch command sent to Steam CLI!");
            Ok(())
        }
        Err(e) => Err(format!("Error: {}", e)),
    }
}