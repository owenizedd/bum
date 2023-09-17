use clap::Parser;
use dirs::home_dir;
use std::path::PathBuf;
use std::error::Error;
mod utils;
use utils::{get_architecture, download_zip, unzip_file};
use std::fs;

#[derive(Parser)]
struct Cli {
    // command: use, install, default
    command: String,
    version: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    println!("Bum - installing bun for version {}...", args.version);
    let _home_path: Option<PathBuf> = home_dir();
    let arch = get_architecture();
    let github_bun_download_url : std::string::String = format!("https://github.com/oven-sh/bun/releases/download/bun-v{}/bun-{}.zip", args.version, arch);
    println!("{}", github_bun_download_url);

    let folder_version_base = "./bun-versions";

     // Create the directory if it doesn't exist
     if !fs::metadata(folder_version_base).is_ok() {
        fs::create_dir_all(folder_version_base)?;
    }

    let zip_file_path = &format!("{}/{}.zip",folder_version_base,args.version);
    let result = download_zip(&github_bun_download_url, &zip_file_path).await;
    match result {
        Ok(()) => {
            
            // Unzip the downloaded file into a folder named after the version
            unzip_file(&zip_file_path, folder_version_base).await?;
            println!("Unzipped to: {}/{}", folder_version_base, args.version);
        },
        Err(err) => eprintln!("Error: {}", err),
    }

    Ok(())
}