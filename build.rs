use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=native/ios/LiquidGlassBridge.swift");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "ios" {
        return;
    }

    let target = env::var("TARGET").unwrap_or_default();
    let sdk = if target.contains("sim") || target.contains("x86_64-apple-ios") {
        "iphonesimulator"
    } else {
        "iphoneos"
    };

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    let source = PathBuf::from("native/ios/LiquidGlassBridge.swift");
    let library = out_dir.join("libliquid_glass_bridge.a");

    let status = Command::new("xcrun")
        .args(["--sdk", sdk, "swiftc"])
        .args(["-parse-as-library", "-emit-library", "-static", "-O"])
        .arg(&source)
        .arg("-o")
        .arg(&library)
        .status()
        .expect("failed to execute xcrun swiftc");

    if !status.success() {
        panic!(
            "Swift bridge build failed for target `{target}` using sdk `{sdk}`. \
             Ensure Xcode command line tools are installed."
        );
    }

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=liquid_glass_bridge");
    println!("cargo:rustc-link-lib=framework=UIKit");
    println!("cargo:rustc-link-lib=framework=Foundation");
}
