use super::utils::{get_active_version, get_github_tags};
use crate::utils::utils;
use async_std::fs::remove_dir_all;
use lazy_static::lazy_static;
use owo_colors::{self, DynColors, OwoColorize};
use resolve_path::PathResolveExt;
use std::borrow::Cow;
use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use utils::check_folder_exists;

lazy_static! {
    pub static ref FOLDER_VERSION_BASE: Cow<'static, Path> = "~/.bum/bun-versions".resolve();
    pub static ref BUN_BIN_PATH: Cow<'static, Path> = "~/.bun/bin/bun".resolve();
}

pub async fn use_bun(version: &str) {
    let arch = utils::get_architecture();

    let active_color: DynColors = "#eea990".parse().unwrap();
    let active_style = owo_colors::Style::new().color(active_color).bold();

    let path_with_version = FOLDER_VERSION_BASE.join(version);

    if check_folder_exists(&path_with_version) {
        let bun_used_path = path_with_version.join(format!("bun-{}/bun", arch));
        match activate_bun(bun_used_path) {
            Ok(()) => println!(
                "Bun {} is activated.",
                format!("v{}", version).style(active_style)
            ),
            _ => println!("Failed to activate Bun v{}", version),
        }
    } else {
        println!("Bum - installing bun for version {}...", version);

        let github_bun_download_url: std::string::String = format!(
            "https://github.com/oven-sh/bun/releases/download/bun-v{}/bun-{}.zip",
            version, arch
        );

        if fs::metadata(FOLDER_VERSION_BASE.to_owned()).is_err() {
            let _ = fs::create_dir_all(FOLDER_VERSION_BASE.to_owned());
        }

        let zip_file_path = FOLDER_VERSION_BASE.join(format!("{}.zip", version));

        let result = utils::download_zip(&github_bun_download_url, &zip_file_path).await;
        match result {
            Ok(()) => {
                let _ = utils::unzip_file(&zip_file_path, &FOLDER_VERSION_BASE).await;

                let bun_used_path = FOLDER_VERSION_BASE
                    .join(version)
                    .join(format!("bun-{}/bun", arch));

                match activate_bun(bun_used_path) {
                    Ok(()) => println!(
                        "Bun {} is activated.",
                        format!("v{}", version).style(active_style)
                    ),
                    _ => println!("Failed"),
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

pub fn activate_bun(bun_used_path: PathBuf) -> Result<(), Box<dyn Error>> {
    fs::remove_file(BUN_BIN_PATH.to_owned()).expect("Failed to delete the current bun bin");
    let mut file_to_copy = File::open(bun_used_path).unwrap();
    let mut file_target = File::create(BUN_BIN_PATH.to_owned()).unwrap();
    io::copy(&mut file_to_copy, &mut file_target).expect("Faield to copy the bun bin");

    fs::metadata(BUN_BIN_PATH.to_owned()).ok().map(|metadata| {
        let mut permissions = metadata.permissions();
        permissions.set_mode(permissions.mode() | 0o111); // Add execute permission

        fs::set_permissions(BUN_BIN_PATH.to_owned(), permissions).unwrap();
        "File is now executable!"
    });

    Ok(())
}

pub async fn remove_bun(version: &str) {
    let result = remove_dir_all(FOLDER_VERSION_BASE.join(version)).await;
    match result {
        Ok(()) => {
            println!("v{} has been removed.", version);
        }
        Err(error) => {
            println!(
                "Failed to remove the version, possibly not installed yet: {}",
                error
            )
        }
    }
}

pub fn display_version_list() {
    let mut versions_list: Vec<String> = Vec::new();
    let result = fs::read_dir(FOLDER_VERSION_BASE.to_owned());

    match result {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                let path_buf: PathBuf = entry.path();
                let path_str = path_buf.to_string_lossy().to_string();

                // Normalize path separators
                let path_str = path_str.replace('\\', "/");

                let version = path_str.split('/').last().unwrap_or_default();
                versions_list.push(version.to_string());
            }
            versions_list.sort();
            versions_list.reverse();
            let active_version = get_active_version();
            let active_color: DynColors = "#eea990".parse().unwrap();
            let active_style = owo_colors::Style::new().color(active_color).bold();
            for version in versions_list {
                if version == active_version {
                    let active_version = format!("{} (active)", version.style(active_style));
                    println!("{} {active_version}", "•".style(active_style));
                } else {
                    println!("• {}", version);
                }
            }
        }
        Err(error) => {
            println!("Failed to read versions: {}", error);
        }
    }
}

pub async fn use_bumrc_version() {
    let bumrc_version = utils::get_bumrc_version();
    match bumrc_version {
        Ok(version) => {
            println!("Using version {} from .bumrc", version);
            use_bun(&version).await;
        }
        Err(e) => {
            println!(
                "No version specified or {}, please use bum use <version> or use -h to print help",
                e
            );
        }
    }
}

pub async fn display_remote_version_list() {
    let tags = get_github_tags("https://api.github.com/repos/oven-sh/bun/tags").await;

    match tags {
        Ok(tags) => {
            for tag in tags {
                let tag_string = format!("{:?}", tag).replace("bun-", "").replace('"', "");
                println!("  {}", tag_string);
            }
        }
        Err(e) => {
            println!("Failed to get remote version list: {}", e);
        }
    }
}
