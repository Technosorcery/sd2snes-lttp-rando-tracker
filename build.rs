extern crate includedir_codegen;

use includedir_codegen::Compression;

fn main() {
    includedir_codegen::start("UI_FILES")
        .dir("ui/dist", Compression::Gzip)
        .build("ui_files.rs")
        .unwrap();
}
