use cmake::Config;

fn main() {
    let dst = Config::new("audio-input").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=audioinput");
}