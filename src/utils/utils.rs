use std::process::Command;
use std::str;
use std::fs;
use std::io;
use reqwest::{self};
use serde_json::Value;
use tokio::fs::File;
use tokio::io::{ AsyncWriteExt, BufWriter};
use std::error::Error as StdError; // Import std::error::Error
use std::path::{Path, PathBuf};
use std::error::Error;
use serde_json::{self};

use zip;
pub fn get_architecture() -> &'static str {
  let output = Command::new("uname")
      .arg("-ms")
      .output()
      .expect("Failed to execute uname");

  let stdout = str::from_utf8(&output.stdout).expect("Failed to convert stdout to string");
  let system_info = stdout.trim();
  match system_info {
      "Darwin x86_64" => "darwin-x64",
      "Darwin arm64" => "darwin-aarch64",
      "Linux aarch64" | "Linux arm64" => "linux-aarch64",
      "Linux x86_64" | _ => "linux-x64",
  }
}

pub async fn download_zip(url: &str, local_path: &str) -> Result<(), Box<dyn StdError>> {

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




pub async fn unzip_file(zip_file_path: &str, output_dir: &str) -> Result<(), Box<dyn Error>> {
    // Extract the version from the ZIP file name (excluding ".zip" suffix)
    let version = Path::new(zip_file_path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or("Invalid ZIP file path")?;

    let output_dir = PathBuf::from(format!("{}/{}", output_dir, version));

    println!("Extracting zip file...");
    
    let zip_file = fs::File::open(zip_file_path).unwrap();

    let mut archive = zip::ZipArchive::new(zip_file).unwrap();
    // zip_extract::extract(Cursor::new(bytes_vec), &output_dir, true)?;
    
    for i in 0..archive.len() {
        let mut file_in_archive = archive.by_index(i).unwrap();
        let mut output_path = match file_in_archive.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        output_path = output_dir.join(output_path);
        if (*file_in_archive.name()).ends_with('/') {
            fs::create_dir_all(&output_path).unwrap();
        }
        else {
            let mut output_file = fs::File::create(&output_path).unwrap();
            io::copy(&mut file_in_archive, &mut output_file).unwrap();

        }
    }

    let _ = fs::remove_file(zip_file_path);
    Ok(())
}


pub fn check_folder_exists(path: &str) -> bool{

    // Use std::fs::metadata to check if the folder exists
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                return true
            } else {
                return false
            }
        }
        Err(_) => {
            return false
        }
    }
}

pub fn get_active_version() -> String {
    //get active version by running bun -v
    let output = Command::new("bun")
        .arg("-v")
        .output()
        .expect("Failed to execute bun -v, is bun installed?");

    let stdout = str::from_utf8(&output.stdout).expect("Failed to convert stdout to string");
    stdout.trim().to_string()
}

pub fn get_bumrc_version() -> Result<String, &'static str> {
    let bumrc_path = Path::new(".bumrc");
    if bumrc_path.exists() {
        let bumrc_version = fs::read_to_string(".bumrc").expect("Failed to read .bumrc, is it a valid file?");
        Ok(bumrc_version.trim().to_string())
    } else {
        Err("No .bumrc file found")
    }
}


pub async fn get_github_tags(url: &str) -> Result<Vec<String>, Box<dyn Error>> {    
    let client = reqwest::Client::builder().user_agent("bum-version-manager-app").build().unwrap();
    let response = client.get(url).send().await?;

    let response_string = response.text().await?;
    let response_json: Value = serde_json::from_str(&response_string).unwrap();

    let tags_vec: Vec<String> = response_json
        .as_array()
        .unwrap()
        .iter()
        .map(|tag| tag["name"].as_str().unwrap().to_string())
        .filter(|tag| tag.starts_with("bun-"))
        .collect();
    Ok(tags_vec)
}
