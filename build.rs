use includedir_codegen;

use includedir_codegen::Compression;
use std::process::Command;

fn main() {
    build_ui_files();
    package_extra_files();
}

fn build_ui_files() {
    if let Ok(_) = std::env::var("SKIP_UI_BUILD") {
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

fn package_extra_files() {
    includedir_codegen::start("UI_FILES")
        .dir("ui/dist", Compression::Gzip)
        .build("ui_files.rs")
        .unwrap();

    includedir_codegen::start("LOGIC_FILES")
        .dir("logic", Compression::Gzip)
        .build("logic_files.rs")
        .unwrap();
}
