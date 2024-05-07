use crate::bun;
use crate::os;
use crate::utils;
use anyhow::Result;
use lazy_static::lazy_static;
use owo_colors::{self, DynColors, OwoColorize};
use resolve_path::PathResolveExt;
use std::borrow::Cow;
#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::fs::remove_dir_all;
use utils::check_folder_exists;

lazy_static! {
    pub static ref FOLDER_VERSION_BASE: Cow<'static, Path> = "~/.bum/bun-versions".resolve();
    pub static ref BUN_BIN_PATH: Cow<'static, Path> = "~/.bun/bin/bun".resolve();
}

fn normalize_version(version: &str) -> String {
    version.replace('v', "")
}

async fn get_bumrc_version() -> Result<String, &'static str> {
    let bumrc_path = Path::new(".bumrc");
    if bumrc_path.exists() {
        let bumrc_version = fs::read_to_string(".bumrc")
            .await
            .expect("Failed to read .bumrc, is it a valid file?");
        Ok(bumrc_version.trim().to_string())
    } else {
        Err("No .bumrc file found")
    }
}

pub async fn use_bun(version: &str) -> Result<()> {
    let version = normalize_version(version);

    let active_color: DynColors = "#eea990".parse().unwrap();
    let active_style = owo_colors::Style::new().color(active_color).bold();

    let path_with_version = FOLDER_VERSION_BASE.join(&version);

    if check_folder_exists(&path_with_version).await {
        let arch = os::get_architecture();

        let bun_used_path = path_with_version.join(format!("bun-{}/bun", arch));
        if let Ok(()) = activate_bun(bun_used_path).await {
            println!(
                "Bun {} is activated.",
                format!("v{}", version).style(active_style)
            );
        } else {
            println!("Failed to activate Bun v{}", version);
        }

        return Ok(());
    }

    println!("Bum - installing bun for version {}...", version);

    if fs::metadata(FOLDER_VERSION_BASE.to_owned()).await.is_err() {
        let _ = fs::create_dir_all(FOLDER_VERSION_BASE.to_owned());
    }

    let zip_file_path = FOLDER_VERSION_BASE.join(format!("{}.zip", version));

    match bun::download_version_to(&version, &zip_file_path).await {
        Ok(bun_path) => match activate_bun(bun_path).await {
            Ok(()) => println!(
                "Bun {} is activated.",
                format!("v{}", version).style(active_style)
            ),
            e => eprintln!("Failed {e:?}"),
        },
        Err(err) => eprintln!("{err}"),
    }

    Ok(())
}

pub async fn activate_bun(bun_used_path: PathBuf) -> Result<()> {
    fs::copy(bun_used_path, BUN_BIN_PATH.to_owned()).await?;

    let metadata = fs::metadata(BUN_BIN_PATH.to_owned()).await?;

    let mut permissions = metadata.permissions();

    #[cfg(not(windows))]
    permissions.set_mode(permissions.mode() | 0o111); // Add execute permission

    fs::set_permissions(BUN_BIN_PATH.to_owned(), permissions)
        .await
        .unwrap();

    Ok(())
}

pub async fn remove(version: &str) {
    let version = normalize_version(version);

    println!("{:?}", FOLDER_VERSION_BASE.join(&version));
    let result = remove_dir_all(FOLDER_VERSION_BASE.join(&version)).await;
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

pub async fn list() -> Result<()> {
    let mut versions_list: Vec<String> = Vec::new();

    match fs::read_dir(FOLDER_VERSION_BASE.to_owned()).await {
        Ok(mut entries) => {
            while let Some(entry) = entries.next_entry().await? {
                let path_buf: PathBuf = entry.path();
                let path_str = path_buf.to_string_lossy().to_string();

                // Normalize path separators
                let path_str = path_str.replace('\\', "/");

                let version = path_str.split('/').last().unwrap_or_default();
                versions_list.push(version.to_string());
            }
            versions_list.sort();
            versions_list.reverse();
            let active_version = bun::get_active_version().await;
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

    Ok(())
}

pub async fn use_bumrc() -> Result<()> {
    let bumrc_version = get_bumrc_version().await;
    match bumrc_version {
        Ok(version) => {
            println!("Using version {} from .bumrc", version);
            use_bun(&version).await?;
        }
        Err(e) => {
            println!(
                "No version specified or {}, please use bum use <version> or use -h to print help",
                e
            );
        }
    }

    Ok(())
}

pub async fn list_remote() {
    let tags = bun::get_github_tags().await;

    match tags {
        Ok(tags) => {
            for tag in tags {
                println!("  {}", tag);
            }
        }
        Err(e) => {
            println!("Failed to get remote version list: {}", e);
        }
    }
}
