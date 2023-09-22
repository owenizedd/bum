
use dirs::home_dir;
use utils::check_folder_exists;
use std::path::PathBuf;
use crate::utils::utils;
use std::fs::{self, File};
use std::error::Error;
use std::io;
use std::os::unix::fs::PermissionsExt;
pub const FOLDER_VERSION_BASE: &str = "./bun-versions";




pub async fn use_bun(version: &str) -> Result<(), Box<dyn Error>> {
  let home_path = home_dir();
  let arch = utils::get_architecture();

  if check_folder_exists(&format!("{}/{}", FOLDER_VERSION_BASE, version)) {
    let bun_used_path = format!("{}/{}/bun-{}/bun", FOLDER_VERSION_BASE, version, arch);
    match activate_bun(bun_used_path, home_path) {
      Ok(()) => println!("Bun v{} is active", version),
      _ => println!("Failed")
    }
  } else {
    println!("Bum - installing bun for version {}...", version);

    let github_bun_download_url : std::string::String = format!("https://github.com/oven-sh/bun/releases/download/bun-v{}/bun-{}.zip", version, arch);




      // Create the directory if it doesn't exist
    if !fs::metadata(FOLDER_VERSION_BASE).is_ok() {
        fs::create_dir_all(FOLDER_VERSION_BASE)?;
    }

    let zip_file_path = &format!("{}/{}.zip",FOLDER_VERSION_BASE,version);
    let result = utils::download_zip(&github_bun_download_url, &zip_file_path).await;
    match result {
        Ok(()) => {
            
            // Unzip the downloaded file into a folder named after the version
            utils::unzip_file(&zip_file_path, FOLDER_VERSION_BASE).await?;
            
            let bun_used_path = format!("{}/{}/bun-{}/bun", FOLDER_VERSION_BASE, version, arch);
            match activate_bun(bun_used_path, home_path) {
              Ok(()) => println!("Bun v{} is active", version),
              _ => println!("Failed")
            }
        },
        Err(err) => eprintln!("Error: {}", err),
    }
  }
  Ok(())

}


pub fn activate_bun(bun_used_path : String, home_path : Option<PathBuf> ) -> Result<(), Box<dyn Error>> {
  
  match home_path {
    Some(path) => {
      path.into_os_string().into_string().ok().and_then(|home_path| { 
        let target_file = format!("{}/.bun/bin/bun", home_path);
        
        println!("{}", target_file);
        let res = fs::remove_file(target_file);
        match res {
          Ok(()) => println!("Changing bun version..."),
          Err(e) => println!("Changing bun version..."),
        }
        
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