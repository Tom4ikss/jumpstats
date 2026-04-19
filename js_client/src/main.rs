#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::collections::HashMap;
use std::time::Duration;
use shared::jump::{BlockThreshold, DistanceThreshold, JumpTypes};
use sysinfo::{System};
use shared::messages::{InitResponse, SubmitJumpRequest, SubmitJumpResponse};
use anyhow::{Result, Error, anyhow};
use chrono::{DateTime, Utc, Duration as ChronoDuration};
use rfd::MessageDialog;
use tokio::io::{AsyncBufReadExt};
use tray_item::{IconSource, TrayItem};
use crate::auth::get_token_file_path_sync;
use crate::game_launch::{attach_to_log, clear_log_file, ensure_running_or_launch_cs2, is_cs2_running, wait_for_cs2_process, wait_for_engine_init};
use crate::sign::sign_request;
use crate::parser::{JumpParser, ParserError, TierConfig};
use crate::update::update_client;

mod parser;
mod auth;
mod config;
mod sign;
mod game_launch;
mod update;
// pub fn save_jump_locally(record: &JumpRecord, output_dir: &str) -> std::io::Result<()> {
//     fs::create_dir_all(output_dir)?;
//
//     let jump_info = &record.info;
//     let file_name = format!("{:?}.json", jump_info.jump_type);
//     let file_path = format!("{}/{}", output_dir, file_name);
//
//     let mut file = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open(file_path)?;
//
//     let json_str = serde_json::to_string(record)?;
//     writeln!(file, "{},", json_str)?;
//     Ok(())
// }

pub struct AppState {
    pub last_steam_username_check_time: Option<DateTime<Utc>>,
}

impl AppState {
    pub fn update_last_username_check_time(&mut self) {
        self.last_steam_username_check_time = Some(Utc::now());
    }

    pub fn needs_sync(&self) -> bool {
        match self.last_steam_username_check_time {
            None => true,
            Some(last_time) => {
                let now = Utc::now();
                now.signed_duration_since(last_time) >= ChronoDuration::hours(3)
            }
        }
    }
}


#[tokio::main]
async fn main() -> Result<()> {

    let exe_name = env!("EXE_NAME");

    update_client(exe_name)
        .map_err(|err| anyhow!("Failed to update client: {}", err))?;

    let mut _tray = TrayItem::new(
        "CS2 Jump Tracker",
        IconSource::Resource("app_icon")
    ).expect("Fatal Error: Icon loading failed");

    _tray.add_label("JumpStat tracker started")?;

    _tray.add_menu_item("Close tracker", || {
        std::process::exit(0);
    })?;

    _tray.add_menu_item("Logout", || {
        match get_token_file_path_sync() {
            Ok(path) => {
                match std::fs::remove_file(&path) {
                    Ok(_) => println!("Logout successfully"),
                    Err(e) => {
                        if e.kind() != std::io::ErrorKind::NotFound {
                            eprintln!("Logout error: {}", e);
                        }
                    }
                }
                std::process::exit(0);
            }
            Err(_) => {
                std::process::exit(0);
            }
        }

    })?;

    _tray.add_menu_item("Discord server", || {
        let _ = open::that("https://discord.gg/PAEaRuPqZZ");
    })?;


    let api_url: &'static str = env!("PUBLIC_URL");

    let log_file_path = config::get_or_ask_log_path();

    let user_token = match auth::get_or_fetch_token(api_url).await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Auth Error: {}", e);
            println!("Press enter to close client...");
            let _ = std::io::stdin().read_line(&mut String::new());
            return Err(Error::from(anyhow!("AuthError")));
        }
    };


    println!("Connecting to API {}...", api_url);
    let init_url = format!("{}/api/init/{}", &api_url, &user_token);

    let http_client = reqwest::Client::new();


    let mut attempts = 0;
    let max_attempts = 5;

    let init_resp: InitResponse = loop {
        let result = http_client
            .get(&init_url)
            .send()
            .await;

        match result {
            Ok(resp) => {
                match resp.json::<InitResponse>().await {
                    Ok(data) => break data,
                    Err(e) => println!("Deserialize error {:#?}", e),
                }
            }
            Err(e) => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(Error::new(e));
                }
                println!("Network error (attempt {}): {}. Reconnecting...", attempts, e);
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    };

    let mut server_thresholds = init_resp.thresholds;
    println!("Successfully got thresholds from server: {:?}", server_thresholds);

    let mut tiers: HashMap<JumpTypes, TierConfig> = HashMap::new();
    tiers.insert(JumpTypes::LJ, TierConfig(265.0, 270.0, 275.0, 280.0, 284.0));
    tiers.insert(JumpTypes::LAJ, TierConfig(160.0, 170.0, 180.0, 190.0, 200.0));
    tiers.insert(JumpTypes::LAH, TierConfig(267.0, 270.0, 273.0, 275.0, 277.0));
    tiers.insert(JumpTypes::WJ, TierConfig(270.0, 275.0, 280.0, 285.0, 290.0));
    tiers.insert(JumpTypes::BH, TierConfig(275.0, 280.0, 286.0, 291.0, 294.0));
    tiers.insert(JumpTypes::MBH, TierConfig(275.0, 280.0, 287.0, 292.0, 295.0));

    let mut user_min = HashMap::new();

    for jt in JumpTypes::iterator() {
        let min_dist = tiers.get(jt).unwrap().0;
        user_min.insert(*jt, DistanceThreshold(min_dist));
    }

    let mut js_always_thresholds = HashMap::new();
    js_always_thresholds.insert(JumpTypes::LJ, DistanceThreshold(200.0));
    js_always_thresholds.insert(JumpTypes::LAJ, DistanceThreshold(100.0));
    js_always_thresholds.insert(JumpTypes::LAH, DistanceThreshold(0.0));
    js_always_thresholds.insert(JumpTypes::WJ, DistanceThreshold(200.0));
    js_always_thresholds.insert(JumpTypes::BH, DistanceThreshold(200.0));
    js_always_thresholds.insert(JumpTypes::MBH, DistanceThreshold(200.0));

    let mut parser = JumpParser::new(tiers, user_min, js_always_thresholds);
    let mut system = System::new();
    let mut state = AppState{ last_steam_username_check_time: None };

    let mut game_was_running = ensure_running_or_launch_cs2(&mut system, &log_file_path).await;

    loop {
        wait_for_cs2_process(&mut system).await;

        if !game_was_running {
            wait_for_engine_init(&log_file_path).await;
        } else {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        println!("{}", log_file_path.display());

        let mut reader = match attach_to_log(&log_file_path).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Fatal error while reading file: {}", e);
                return Err(e);
            }
        };

        println!("Waiting for first jump...");

        let mut line = String::new();

        let mut is_session_tainted = false;

        loop {
            line.clear();

            match reader.read_line(&mut line).await {
                Ok(0) => {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    //let _ = reader.seek(SeekFrom::Current(0)).await;
                    if !is_cs2_running(&mut system) {
                        println!("CS2 closed...");
                        break;
                    }
                }
                Ok(_) => {
                    match parser.process_line(&line) {
                        Ok(record) => {
                            let info = &record.info;
                            let amount = info.amount;
                            let jump_type = info.jump_type;
                            let min_required = server_thresholds.get(&info.jump_type).copied().unwrap_or((DistanceThreshold(0.0), BlockThreshold(0)));
                            let jt_str: &str = jump_type.into();
                            println!("New jump detected: {} units via {} ({:?} units or {:?} block to top 3 needed)", amount, jt_str, min_required.0, min_required.1);

                            if is_session_tainted && jump_type != JumpTypes::LJ {
                                continue;
                            }

                            let is_new_distance = DistanceThreshold(amount) > server_thresholds.get(&jump_type).map(|t| t.0).unwrap_or(DistanceThreshold(0.0));
                            let is_new_block = if let Some(block) = record.summary.block {
                                BlockThreshold(block) > server_thresholds.get(&jump_type).map(|t| t.1).unwrap_or(BlockThreshold(0))
                            } else {
                                false
                            };

                            if is_new_distance || is_new_block {
                                println!("New top 3, sending to server...");

                                let payload = SubmitJumpRequest {
                                    user_token: user_token.clone(),
                                    steam_username: record.info.steam_username.clone(),
                                    tier: record.info.tier.clone(),
                                    jump_type,
                                    block: record.summary.block,
                                    record,
                                    amount,
                                    is_js_always: is_session_tainted,
                                };

                                let payload_json = serde_json::to_string(&payload)?;
                                let (timestamp, signature) = sign_request(&payload_json);

                                let res = http_client.post(format!("{}/api/submit", api_url))
                                    .header("X-Timestamp", timestamp)
                                    .header("X-Signature", signature)
                                    .header("Content-Type", "application/json")
                                    .body(payload_json)
                                    .send()
                                    .await;

                                match res {
                                    Ok(response) if response.status().is_success() => {
                                        if let Ok(submit_res) = response.json::<SubmitJumpResponse>().await {
                                            if let Some(username) = submit_res.valid_username {
                                                println!("Valid username: {}", username);
                                                parser.set_valid_username(Some(username));
                                                state.update_last_username_check_time();
                                            }
                                            println!("Jump saved successfully!. New min distance for {} top 3 is: {:?} (units, block)", jt_str, submit_res.new_threshold);
                                            server_thresholds.insert(jump_type, submit_res.new_threshold);
                                        }
                                    }
                                    Ok(response) => {
                                        eprintln!("Error while saving jump: {}", response.status());
                                    }
                                    Err(e) => {
                                        eprintln!("Network Error: {}", e);
                                    }
                                }
                            }
                        }
                        Err(ParserError::JSAlwaysError) => {
                            println!("JS Always Error");
                            is_session_tainted = true;
                        }
                        Err(ParserError::InvalidUsernameError) => {
                            if state.needs_sync() {
                                parser.set_valid_username(None);
                            }
                        }
                        Err(_) => {}
                    }
                }
                Err(e) => {
                    eprintln!("Read Error: {}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }

        drop(reader);

        clear_log_file(&log_file_path).await;

        game_was_running = false;
    }
}