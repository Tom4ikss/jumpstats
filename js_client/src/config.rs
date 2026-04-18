use std::{env, fs};
use std::fs::OpenOptions;
use std::path::PathBuf;
use directories::ProjectDirs;
use rfd::{FileDialog, MessageDialog, MessageLevel};
// pub fn show_welcome_dialog() {
//     let res = MessageDialog::new()
//         .set_title("Now please read the basics of using the application.")
//         .set_description(
//             "1. Jumpstats tracking work only on Cybershoke kz servers.\n\
//             2. You will need to login via steam openid.\n\
//             3. You need to start cs2 only through this app or yourself with \"-condebug\" param\n\
//             4. If you accidentally logged into a different account (not the one the game was launched through) use \"logout\" option in the tray\n\
//             5. I strongly don't recommend to play with !jsalways on.\n\
//             6. If you decide to use it anyway only Long Jumps will be tracked and server records will be additionally moderated.\n\
//             7. To switch from jsalways mode to default you need to off !jsalways and restart app\n\
//             8. For using autohotkey/autostrafes you will be banned forever (but you can freely use nulls, -w, crouch, etc. configs)\n\
//             9. For a complete user experience you need to join discord server (use button below)\n\
//             10. Thank you for downloading this app, I hope you will find it useful. My github: Tom4ikss (use button below)"
//         )
//         .set_buttons(MessageButtons::YesNoCancelCustom(
//             "Discord ".into(),
//             "Github".into(),
//             "Close".into(),
//         ))
//         .set_level(MessageLevel::Warning)
//         .show();
//
//     match res {
//         MessageDialogResult::Yes => {
//             let _ = open::that("https://discord.gg/PAEaRuPqZZ");
//         }
//         MessageDialogResult::No => {
//             let _ = open::that("https://github.com/Tom4ikss");
//         }
//         _ => {}
//     }
// }


fn show_guide() {
    let html_content = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8">
        <title>CS2 Jumpstats tracker guide</title>
        <style>
            body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; padding: 40px; background: #1e1e1e; color: #fff; max-width: 600px; margin: 0 auto; }
            h2 { color: #4DA8DA; }
            a { color: white; text-decoration: none; }
            .btn { display: inline-block; padding: 5px 16px; background: #2b2b2b; border: 1px solid #444; border-radius: 6px; margin-right: 10px; margin-top: 20px; transition: 0.2s; }
            .btn:hover { background: #3a3a3a; border-color: #666; }
            .discord { border-left: 4px solid #5865F2; }
            .github { border-left: 4px solid #fff; }
        </style>
    </head>
    <body>
        <h2>Now please read the basics of using the application.</h2>
        <p>1. Jumpstats tracking work only on Cybershoke kz servers.</p>
        <p>2. You will need to login via steam openid.</p>
        <p>3. You need to start cs2 only through this app or yourself with "-condebug" param</p>
        <p>4. If you accidentally logged into a different account (not the one the game was launched through) use "logout" option in the tray</p>
        <p>5. I strongly don't recommend to play with !jsalways on.</p>
        <p>6. If you decide to use it anyway only Long Jumps will be tracked and server records will be additionally moderated.</p>
        <p>7. To switch from jsalways mode to default you need to off !jsalways and restart app</p>
        <p>8. For using autohotkey/autostrafes you will be banned forever (but you can freely use nulls, -w, crouch, etc. configs)</p>
        <p>9. For a complete user experience you need to join <a href="https://discord.gg/PAEaRuPqZZ" class="btn discord">Discord Server</a></p>
        <p>10. Thank you for downloading this app, I hope you will find it useful. <a href="https://github.com/Tom4ikss" class="btn github">My github</a></p>
        <br/>
        <p>Now you can return to the app</p>
    </body>
    </html>
    "#;

    let mut temp_path = env::temp_dir();
    temp_path.push("cs2_tracker_guide.html");

    let _ = fs::write(&temp_path, html_content);

    if let Err(e) = open::that(&temp_path) {
        eprintln!("Failed to open browser: {}", e);
    }

    MessageDialog::new()
        .set_title("CS2 Tracker Guide")
        .set_description("Confirm that you have read the guide.")
        .set_level(MessageLevel::Info)
        .show();
}

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
        .set_title("Welcome to CS2 Jump Tracker")
        .set_description(
            "Now we need to perform the initial setup\n\nYou need to provide path to Counter-Strike Global Offensive folder.\n\ne.g. C:/ProgramFiles(x86)/Steam/steamapps/common/Counter-Strike Global Offensive"
        )
        .set_level(MessageLevel::Info)
        .show();
    
    let chosen_path = FileDialog::new()
        .set_title("Choose Counter-Strike Global Offensive folder path")
        .pick_folder();

    match chosen_path {
        Some(mut path) => {


            path.push("game");
            path.push("csgo");
            path.push("console.log");

            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).expect("Fatal error while creating folder");
                }
            }

            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(false)
                .open(&path)
                .expect("Error while creating console.log");

            fs::write(&path_file, path.to_string_lossy().to_string())
                .expect("Failed to save log file path");
            println!("Path saved!");

            show_guide();

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

