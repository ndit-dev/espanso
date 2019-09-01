extern crate cmake;
use cmake::Config;
use std::path::PathBuf;

/* OS SPECIFIC CONFIGS */

#[cfg(target_os = "windows")]
fn get_config() -> PathBuf {
    Config::new("native/libwinbridge").build()
}

#[cfg(target_os = "linux")]
fn get_config() -> PathBuf {
    Config::new("native/liblinuxbridge").build()
}

/*
    OS CUSTOM CARGO CONFIG LINES
    Note: this is where linked libraries should be specified.
*/

#[cfg(target_os = "windows")]
fn print_config()  {
    println!("cargo:rustc-link-lib=static=winbridge");
    println!("cargo:rustc-link-lib=static=user32");
}

#[cfg(target_os = "linux")]
fn print_config() {
    println!("cargo:rustc-link-lib=static=linuxbridge");
    println!("cargo:rustc-link-lib=dylib=X11");
    println!("cargo:rustc-link-lib=dylib=Xtst");
}

fn main()
{
    let dst = get_config();

    println!("cargo:rustc-link-search=native={}", dst.display());
    print_config();
}