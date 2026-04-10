use std::{env, fs};
use std::path::Path;

fn main() {

    println!("cargo:rerun-if-env-changed=CS2_TRACKER_SECRET");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_secret.rs");

    let secret = env::var("CS2_TRACKER_SECRET")
        .unwrap_or_else(|_| "COMMUNITY_UNRANKED_KEY".to_string());

    let generated_code = format!(
        "pub fn get_secret() -> String {{\n    obfstr::obfstr!(\"{}\").to_string()\n}}",
        secret
    );

    fs::write(&dest_path, generated_code).unwrap();

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon_with_id("kz.ico", "app_icon");
        res.compile().unwrap();
    }
}