use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let adlx_sdk = PathBuf::from(
        r"C:\Users\SleepySlezy\Downloads\ADLX-main\ADLX-main\SDK"
    );

    let include = adlx_sdk.join("Include");
    let helper = adlx_sdk.join("ADLXHelper").join("Windows").join("Cpp");
    let platform = adlx_sdk.join("Platform").join("Windows");

    println!("cargo:rerun-if-changed={}", include.display());
    println!("cargo:rerun-if-changed={}", helper.display());
    println!("cargo:rerun-if-changed={}", platform.display());

cc::Build::new()
    .cpp(true)
    .file("src/adlx_bridge.cpp")
    .file(helper.join("ADLXHelper.cpp"))
    .file(platform.join("WinAPIs.cpp"))
    .include(&include)
    .include(&helper)
    .include(&platform)
    .compile("adlx_bridge");

    println!(
        "cargo:rustc-link-search=native={}",
        manifest_dir.join("target").display()
    );
}