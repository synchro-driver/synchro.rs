use cmake::Config;
use std::path::Path;
use std::fs;

fn main() {
    let dst = Config::new("audio-input").build();

    let build_log_path = Path::new(".").join("runtime.log");
    fs::write(&build_log_path, format!("Target native lib path= {}\n", dst.display().to_string())).unwrap();

    println!("cargo:rustc-link-search=native={}/bin", dst.display());
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=audioinput");

    println!("cargo:rerun-if-changed=audio-input/audio.c");
}