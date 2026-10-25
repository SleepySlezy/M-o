use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();

    println!("cargo:rerun-if-changed=src/adlx_bridge.cpp");
    println!("cargo:rerun-if-env-changed=ADLX_SDK_PATH");

    if !target.contains("windows") {
        return;
    }

    let mut build = cc::Build::new();
    build.cpp(true).file("src/adlx_bridge.cpp");

    if let Some(sdk_path) = env::var_os("ADLX_SDK_PATH") {
        let sdk = PathBuf::from(sdk_path);
        let include = sdk.join("Include");
        let helper = sdk.join("ADLXHelper").join("Windows").join("Cpp");
        let platform = sdk.join("Platform").join("Windows");
        let helper_cpp = helper.join("ADLXHelper.cpp");
        let platform_cpp = platform.join("WinAPIs.cpp");

        if !include.is_dir() || !helper_cpp.is_file() || !platform_cpp.is_file() {
            panic!("Invalid ADLX SDK root: {}", sdk.display());
        }

        println!("cargo:rerun-if-changed={}", include.display());
        println!("cargo:rerun-if-changed={}", helper_cpp.display());
        println!("cargo:rerun-if-changed={}", platform_cpp.display());

        build
            .file(helper_cpp)
            .file(platform_cpp)
            .include(include)
            .include(helper)
            .include(platform);
    } else {
        println!("cargo:warning=ADLX_SDK_PATH is unset; building without ADLX telemetry.");
    }

    build.compile("adlx_bridge");
}
