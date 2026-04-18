use std::{env, fs};
use std::path::Path;

fn main() {

    println!("cargo:rerun-if-env-changed=SAVE_JUMP_SECRET_KEY");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_secret.rs");

    let secret = match env::var("SAVE_JUMP_SECRET_KEY") {
        Ok(val) => {
            println!("cargo:warning=Successfully get secret from SAVE_JUMP_SECRET_KEY");
            val
        }
        Err(_) => {
            println!("cargo:error=ERROR: Secret not found.");
            std::process::exit(1);
        }
    };

    let generated_code = format!(
        "pub fn get_secret() -> String {{\n    obfstr::obfstr!(\"{}\").to_string()\n}}",
        secret
    );

    fs::write(&dest_path, generated_code).unwrap();

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon_with_id("kz.ico", "app_icon");
        res.compile().unwrap();
    }
}