use anyhow::Result;
use reqwest;
use std::env::consts::{ARCH, OS};
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::{AsyncWriteExt, BufWriter};
use zip;

pub fn get_architecture() -> String {
    let os = if OS == "macos" { "darwin" } else { OS };

    let arch = match ARCH {
        "x86_64" => "x64",
        "arm" => "aarch64",
        _ => ARCH,
    };

    format!("{os}-{arch}")
}

pub async fn download_zip(url: &str, local_path: &Path) -> Result<()> {
    let client = reqwest::Client::new();

    let response = client.get(url).send().await?;

    if response.status().is_success() {
        let file = File::create(local_path).await?;
        let mut writer = BufWriter::new(file);

        // Read the entire response body into a Vec<u8>
        let bytes = response.bytes().await?;

        // Write the bytes to the local file
        writer.write_all(&bytes).await?;

        writer.flush().await?;
    } else {
        println!("HTTP request was not successful: {:?}", response.status());
    }
    Ok(())
}

// TODO: Refactor this function when zip crate has async: https://github.com/zip-rs/zip2/pull/73
pub async fn unzip_file(zip_file_path: &Path, output_dir: &Path) -> Result<()> {
    // Extract the version from the ZIP file name (excluding ".zip" suffix)
    let version = Path::new(zip_file_path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("Invalid ZIP file path");

    let output_dir = output_dir.join(version);

    println!("Extracting zip file...");

    let zip_file = std::fs::File::open(zip_file_path)?;

    let mut archive = zip::ZipArchive::new(zip_file)?;
    // zip_extract::extract(Cursor::new(bytes_vec), &output_dir, true)?;

    for i in 0..archive.len() {
        let mut file_in_archive = archive.by_index(i).unwrap();
        let mut output_path = match file_in_archive.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        output_path = output_dir.join(output_path);
        if (*file_in_archive.name()).ends_with('/') {
            fs::create_dir_all(&output_path).await?;
        } else {
            let mut output_file = std::fs::File::create(&output_path)?;
            std::io::copy(&mut file_in_archive, &mut output_file)?;
        }
    }

    fs::remove_file(zip_file_path).await?;

    Ok(())
}

pub async fn check_folder_exists(path: &PathBuf) -> bool {
    fs::metadata(path)
        .await
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}
