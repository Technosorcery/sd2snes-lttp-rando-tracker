use anyhow::Result;
use std::process::Command;
use vergen::{
    vergen,
    Config,
};

fn main() -> Result<()> {
    build_ui_files();

    let mut vergen_config = Config::default();
    *vergen_config.git_mut().semver_dirty_mut() = Some("-dirty");
    vergen(vergen_config)
}

fn build_ui_files() {
    if std::env::var("BUILD_UI").is_err() {
        return;
    }

    let ui_dir = match std::env::current_dir() {
        Ok(mut d) => {
            d.push("ui");
            d
        }
        Err(e) => panic!("Could not get current directory: {}", e),
    };

    let yarn_command = match std::env::var("YARN_PATH") {
        Ok(c) => c,
        _ => "yarn".to_string(),
    };
    println!("yarn command: {}", &yarn_command);
    let ui_build_status = Command::new(yarn_command)
        .arg("build")
        .current_dir(ui_dir)
        .status()
        .expect("Failed to build UI");
    if !ui_build_status.success() {
        panic!("Could not build UI");
    }
}
