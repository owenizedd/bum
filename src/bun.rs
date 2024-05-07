use crate::{commands::FOLDER_VERSION_BASE, os};
use anyhow::{bail, Result};
use reqwest::StatusCode;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::{
    fs::{self, create_dir_all, File},
    io::{AsyncWriteExt, BufWriter},
    process::Command,
};

const BUN_GITHUB_TAGS_URL: &str = "https://api.github.com/repos/oven-sh/bun/tags";
const BUN_BIN_NAME: &str = "bun";

pub async fn get_active_version() -> String {
    //get active version by running bun -v
    let output = Command::new("bun")
        .arg("-v")
        .output()
        .await
        .expect("Failed to execute bun -v, is bun installed?");

    let stdout = std::str::from_utf8(&output.stdout).expect("Failed to convert stdout to string");
    stdout.trim().to_string()
}

#[derive(Deserialize)]
struct Tag {
    name: String,
}

pub async fn get_github_tags() -> Result<Vec<String>> {
    let client = reqwest::Client::builder()
        .user_agent("bum-version-manager-app")
        .build()?;

    let response = client.get(BUN_GITHUB_TAGS_URL).send().await?;

    Ok(response
        .json::<Vec<Tag>>()
        .await?
        .into_iter()
        .filter_map(|tag| {
            if tag.name.starts_with("bun-") {
                Some(tag.name.replace("bun-", ""))
            } else {
                None
            }
        })
        .collect())
}

pub async fn download_version_to(version: &str, to_path: &Path) -> Result<PathBuf> {
    let client = reqwest::Client::new();

    let arch = os::get_architecture();

    let url = format!(
        "https://github.com/oven-sh/bun/releases/download/bun-v{}/bun-{}.zip",
        version, arch
    );

    let response = client.get(url).send().await?;

    match response.status() {
        StatusCode::OK => {
            let file = File::create(to_path).await?;
            let mut writer = BufWriter::new(file);

            // Read the entire response body into a Vec<u8>
            let bytes = response.bytes().await?;

            // Write the bytes to the local file
            writer.write_all(&bytes).await?;

            writer.flush().await?;
        }
        StatusCode::NOT_FOUND => {
            bail!("Version \"{version}\" doesn't exist");
        }
        e => {
            bail!("HTTP request was not successful: {e}");
        }
    }

    extract_bun_bin_of_zip(&to_path, &FOLDER_VERSION_BASE).await
}

// TODO: Refactor this function when zip crate has async: https://github.com/zip-rs/zip2/pull/73
async fn extract_bun_bin_of_zip(zip_file_path: &Path, output_dir: &Path) -> Result<PathBuf> {
    // Extract the version from the ZIP file name (excluding ".zip" suffix)
    let version = Path::new(zip_file_path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("Invalid ZIP file path");

    let output_dir = output_dir.join(version);

    println!("Extracting zip file...");

    let zip_file = std::fs::File::open(zip_file_path)?;

    let mut archive = zip::ZipArchive::new(zip_file)?;

    for i in 0..archive.len() {
        let mut file_in_archive = archive.by_index(i).unwrap();
        let output_path = match file_in_archive.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if !output_path.ends_with(BUN_BIN_NAME) {
            continue;
        }

        create_dir_all(output_dir.clone()).await?;

        let output_path = output_dir.join(BUN_BIN_NAME);
        let mut output_file = std::fs::File::create(output_path.clone())?;
        std::io::copy(&mut file_in_archive, &mut output_file)?;

        fs::remove_file(zip_file_path).await?;

        return Ok(output_path);
    }

    bail!("Failed to find Bun binary in the zip file")
}
