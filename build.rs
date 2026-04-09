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
    let (sdk, swift_target) = match target.as_str() {
        "aarch64-apple-ios" => ("iphoneos", "arm64-apple-ios"),
        "aarch64-apple-ios-sim" => ("iphonesimulator", "arm64-apple-ios-simulator"),
        "x86_64-apple-ios" => ("iphonesimulator", "x86_64-apple-ios-simulator"),
        _ => {
            panic!("unsupported iOS target `{target}` for Swift bridge build");
        }
    };

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    let source = PathBuf::from("native/ios/LiquidGlassBridge.swift");
    let library = out_dir.join("libliquid_glass_bridge.a");
    let sdk_path_output = Command::new("xcrun")
        .args(["--sdk", sdk, "--show-sdk-path"])
        .output()
        .expect("failed to resolve Apple SDK path with xcrun");
    if !sdk_path_output.status.success() {
        panic!("failed to resolve SDK path for `{sdk}`");
    }
    let sdk_path = String::from_utf8(sdk_path_output.stdout)
        .expect("xcrun returned non-utf8 sdk path")
        .trim()
        .to_string();

    let status = Command::new("xcrun")
        .args(["--sdk", sdk, "swiftc"])
        .args(["-target", swift_target, "-sdk", &sdk_path])
        .args(["-parse-as-library", "-emit-library", "-static", "-O"])
        .arg(&source)
        .arg("-o")
        .arg(&library)
        .status()
        .expect("failed to execute xcrun swiftc");

    if !status.success() {
        panic!(
            "Swift bridge build failed for target `{target}` (swift target `{swift_target}`) using sdk `{sdk}`. \
             Ensure Xcode command line tools are installed."
        );
    }

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=liquid_glass_bridge");
    println!("cargo:rustc-link-lib=framework=UIKit");
    println!("cargo:rustc-link-lib=framework=Foundation");
}
