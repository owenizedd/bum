use std::path::PathBuf;
use tokio::fs::{self};

pub async fn check_folder_exists(path: &PathBuf) -> bool {
    fs::metadata(path)
        .await
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}
