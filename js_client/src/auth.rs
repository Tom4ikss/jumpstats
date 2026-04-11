use directories::ProjectDirs;
use tokio::net::{TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs;
use std::net::SocketAddr;
use std::path::PathBuf;

pub fn get_token_file_path_sync() -> Result<PathBuf, String> {

    if let Some(proj_dirs) = ProjectDirs::from("com", "CS2", "JumpTracker") {
        let dir = proj_dirs.data_dir();

        if !dir.exists() {
            std::fs::create_dir_all(dir).map_err(|e| format!("Failed to create dir: {}", e))?;
        }

        Ok(dir.join("token"))
    } else {
        Err("Failed to find AppData dir".to_string())
    }
}

pub async fn get_token_file_path() -> Result<PathBuf, String> {

    if let Some(proj_dirs) = ProjectDirs::from("com", "CS2", "JumpTracker") {
        let dir = proj_dirs.data_dir();

        if !dir.exists() {
            fs::create_dir_all(dir).await.map_err(|e| format!("Failed to create dir: {}", e))?;
        }

        Ok(dir.join("token"))
    } else {
        Err("Failed to find AppData dir".to_string())
    }
}

pub async fn get_or_fetch_token(api_url: &str) -> Result<String, Box<dyn std::error::Error>> {

    let token_path = get_token_file_path().await?;

    if token_path.exists() {
        match fs::read_to_string(&token_path).await {
            Ok(token) if !token.trim().is_empty() => {
                println!("Token found: {}", token.trim());
                return Ok(token.trim().to_string());
            }
            Ok(_) => println!("Token file is empty. Auth required..."),
            Err(e) => println!("Error while reading token file: {:?}. Auth required...", e),
        }
    } else {
        println!("Token not found. Auth required...");
    }
    
    let port = 1337;
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();
    let listener = TcpListener::bind(&addr).await?;
    
    let login_url = format!("{}/api/auth/steam/login?port={}", api_url, port);
    webbrowser::open(&login_url)?;

    println!("Waiting for auth...");
    
    let (mut socket, _) = listener.accept().await?;
    let mut buffer = [0; 1024];
    socket.read(&mut buffer).await?;

    let request = String::from_utf8_lossy(&buffer);
    
    let token = extract_token(&request).ok_or("Failed to extract token")?;
    
    let html_response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n\
        <html><body style='background: #121212; color: #ffffff; font-family: sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0;'>\
        <div style='text-align: center;'>\
        <h1>Auth complete!</h1>\
        <p>You can close this tab.</p>\
        <script>setTimeout(() => window.close(), 3000);</script>\
        </div></body></html>";

    socket.write_all(html_response.as_bytes()).await?;

    fs::write(&token_path, &token).await?;
    println!("Token saved, login...");

    Ok(token)
}

fn extract_token(request: &str) -> Option<String> {
    let first_line = request.lines().next()?;
    let path = first_line.split_whitespace().nth(1)?;

    if path.starts_with("/?token=") {
        Some(path.replace("/?token=", ""))
    } else {
        None
    }
}