
use async_std::fs::remove_dir_all;
use dirs::home_dir;
use utils::check_folder_exists;
use std::path::PathBuf;
use crate::utils::utils;
use std::fs::{self, File};
use std::error::Error;
use std::io;
use std::os::unix::fs::PermissionsExt;
use owo_colors::{self, OwoColorize, DynColors};
use super::utils::get_active_version;
pub const FOLDER_VERSION_BASE: &str = "./bun-versions";



pub async fn use_bun(version: &str) -> () {
  let home_path = home_dir();
  let arch = utils::get_architecture();
  
  let active_color: DynColors = "#eea990".parse().unwrap();
  let active_style = owo_colors::Style::new().color(active_color).bold();

  if check_folder_exists(&format!("{}/{}", FOLDER_VERSION_BASE, version)) {
    let bun_used_path = format!("{}/{}/bun-{}/bun", FOLDER_VERSION_BASE, version, arch);
    match activate_bun(bun_used_path, home_path) {
      Ok(()) => println!("Bun {} is activated.", format!("v{}",version).style(active_style)),
      _ => println!("Failed to activate Bun v{}", version)
    }
  } else {
    println!("Bum - installing bun for version {}...", version);

    let github_bun_download_url : std::string::String = format!("https://github.com/oven-sh/bun/releases/download/bun-v{}/bun-{}.zip", version, arch);

    if !fs::metadata(FOLDER_VERSION_BASE).is_ok() {
        let _ = fs::create_dir_all(FOLDER_VERSION_BASE);
    }

    let zip_file_path = &format!("{}/{}.zip",FOLDER_VERSION_BASE,version);
    let result = utils::download_zip(&github_bun_download_url, &zip_file_path).await;
    match result {
        Ok(()) => {
            let _ = utils::unzip_file(&zip_file_path, FOLDER_VERSION_BASE).await;
            
            let bun_used_path = format!("{}/{}/bun-{}/bun", FOLDER_VERSION_BASE, version, arch);
            match activate_bun(bun_used_path, home_path) {
              Ok(()) => println!("Bun {} is activated.", format!("v{}",version).style(active_style)),
              _ => println!("Failed")
            }
        },
        Err(err) => eprintln!("Error: {}", err),
    }
  }
 

}


pub fn activate_bun(bun_used_path : String, home_path : Option<PathBuf> ) -> Result<(), Box<dyn Error>> {
  
  match home_path {
    Some(path) => {
      path.into_os_string().into_string().ok().and_then(|home_path| { 
        let target_file = format!("{}/.bun/bin/bun", home_path);

        let _ = fs::remove_file(target_file);
        let mut file_to_copy = File::open(bun_used_path).unwrap();
        let target_path = &format!("{}/.bun/bin/bun", home_path);
        let mut file_target = File::create(target_path).unwrap();
        let success = io::copy(&mut file_to_copy, &mut file_target);

        fs::metadata(target_path).ok().and_then(|metadata| {

            let mut permissions = metadata.permissions();
            permissions.set_mode(permissions.mode() | 0o111); // Add execute permission

            fs::set_permissions(target_path, permissions).unwrap();
            Some("File is now executable!")
        });
        Some(success)

      });
    },
    None => println!("Failed to get home path")
  }
  Ok(())
}

pub async fn remove_bun(version: &str) { 
  // let home_path = home_dir();

  let result = remove_dir_all(format!("{}/{}", FOLDER_VERSION_BASE, version)).await;
  match result { 
    Ok(()) => {
      println!("v{} has been removed.", version);
    }
    Err(error) => {
      println!("Failed to remove the version, possibly not installed yet: {}", error)
    }
  }
}

pub fn display_versions_list() {
  let mut versions_list: Vec<String> = Vec::new();
  let result = fs::read_dir(FOLDER_VERSION_BASE);
  
  match result {
      Ok(entries) => {
          for entry in entries {
              let entry = entry.unwrap();
              let path_buf: PathBuf = entry.path();
              let path_str = path_buf.to_string_lossy().to_string(); 

              // Normalize path separators
              let path_str = path_str.replace("\\", "/"); 

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
                  println!("{} {}", "•".style(active_style), format!("{} (active)", version.style(active_style)));
              } else {
                  println!("• {}", version);
              }
          }
      },
      Err(error) => {
          println!("Failed to read versions: {}", error);
      }
  }
}

pub async fn use_bumrc_version(){
  let bumrc_version = utils::get_bumrc_version();
  match bumrc_version {
    Ok(version) => {
      println!("Using version {} from .bumrc", version);
      use_bun(&version).await;
    },
    Err(e) => {
      println!("No version specified or {}, please use bum use <version> or use -h to print help", e);
    }
  }
}