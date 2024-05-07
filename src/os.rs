use std::env::consts::{ARCH, OS};

pub fn get_architecture() -> String {
    let os = if OS == "macos" { "darwin" } else { OS };

    let arch = match ARCH {
        "x86_64" => "x64",
        "arm" => "aarch64",
        _ => ARCH,
    };

    format!("{os}-{arch}")
}
