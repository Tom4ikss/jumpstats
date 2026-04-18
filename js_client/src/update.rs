use std::fs;
use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use self_update::cargo_crate_version;

pub fn update_client(program_name: &str) -> Result<(), Box<dyn std::error::Error>> {

    let old_exe = std::env::current_exe()?.with_extension("exe.old");
    if old_exe.exists() {
        let _ = fs::remove_file(old_exe);
    }

    let updater = self_update::backends::github::Update::configure()
        .repo_owner("Tom4ikss")
        .repo_name("jumpstats")
        .target("windows.zip")
        .bin_name(&format!("{}.exe", program_name))
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?;

    let latest_release = updater.get_latest_release()?;

    let is_newer = self_update::version::bump_is_greater(
        cargo_crate_version!(),
        &latest_release.version
    ).unwrap_or(false);

    if is_newer {
        let res = MessageDialog::new()
            .set_title("New version available")
            .set_description(format!("New version {} available, current version {}.\n You need to install latest version to use app <3.\n Update now?", latest_release.version, cargo_crate_version!()))
            .set_buttons(MessageButtons::YesNo)
            .set_level(MessageLevel::Info)
            .show();

        match res {
            MessageDialogResult::Yes => {

                updater.update()?;

                MessageDialog::new()
                    .set_title("Version updated")
                    .set_description("New version successfully installed. Please restart app.")
                    .set_level(MessageLevel::Info)
                    .show();

                std::process::exit(0);
            }
            MessageDialogResult::No => {

                MessageDialog::new()
                    .set_title("Outdated")
                    .set_description("Sorry you can't use current version.")
                    .set_level(MessageLevel::Error)
                    .show();

                std::process::exit(0);
            }
            _ => panic!("Unreachable!")
        }

    }

    Ok(())
}