
use dirs::home_dir;
use std::path::PathBuf;
use crate::utils::utils;
use std::fs;

pub async fn install_bun(version: &str) -> Result<(), Box<dyn Error>> {
  println!("Bum - installing bun for version {}...", version);
  let _home_path: Option<PathBuf> = home_dir();
  let arch = utils::get_architecture();
  let github_bun_download_url : std::string::String = format!("https://github.com/oven-sh/bun/releases/download/bun-v{}/bun-{}.zip", version, arch);
  println!("{}", github_bun_download_url);

  let folder_version_base = "./bun-versions";

    // Create the directory if it doesn't exist
    if !fs::metadata(folder_version_base).is_ok() {
      fs::create_dir_all(folder_version_base)?;
  }

  let zip_file_path = &format!("{}/{}.zip",folder_version_base,version);
  let result = utils::download_zip(&github_bun_download_url, &zip_file_path).await;
  match result {
      Ok(()) => {
          
          // Unzip the downloaded file into a folder named after the version
          utils::unzip_file(&zip_file_path, folder_version_base).await?;
          println!("Unzipped to: {}/{}", folder_version_base, version);
      },
      Err(err) => eprintln!("Error: {}", err),
  }

  Ok(())
}