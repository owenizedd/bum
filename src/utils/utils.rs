use std::process::Command;
use std::str;
use std::fs;
use std::io;
use reqwest::{self};
use tokio::fs::File;
use tokio::io::{ AsyncWriteExt, BufWriter};
use std::error::Error as StdError; // Import std::error::Error
use std::path::{Path, PathBuf};
use std::error::Error;


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