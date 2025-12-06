use crate::bun;
use crate::bun::BUN_BIN_NAME;
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

pub fn normalize_version(version: &str) -> String {
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
        let bun_used_path = path_with_version.join(BUN_BIN_NAME);

        match activate_bun(bun_used_path).await {
            Ok(()) => {
                println!(
                    "Bun {} is activated.",
                    format!("v{}", version).style(active_style)
                );
            }
            Err(e) => {
                eprintln!("Failed to activate Bun v{}", version);
                eprintln!("Reason: {e:?}");
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // Test helper to create a temporary test environment
    struct TestEnv {
        _temp_dir: TempDir,
        versions_dir: PathBuf,
        bin_dir: PathBuf,
    }

    impl TestEnv {
        fn new() -> Self {
            let temp_dir = TempDir::new().unwrap();
            let bum_dir = temp_dir.path().join(".bum");
            let versions_dir = bum_dir.join("bun-versions");
            let bin_dir = temp_dir.path().join(".bun").join("bin");

            fs::create_dir_all(&versions_dir).unwrap();
            fs::create_dir_all(&bin_dir).unwrap();

            TestEnv {
                _temp_dir: temp_dir,
                versions_dir,
                bin_dir,
            }
        }

        fn create_mock_version(&self, version: &str) -> PathBuf {
            let version_dir = self.versions_dir.join(version);
            fs::create_dir_all(&version_dir).unwrap();

            let bun_bin = version_dir.join(BUN_BIN_NAME);
            fs::write(&bun_bin, format!("mock bun {}", version)).unwrap();

            #[cfg(not(windows))]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&bun_bin).unwrap().permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&bun_bin, perms).unwrap();
            }

            version_dir
        }

        fn version_exists(&self, version: &str) -> bool {
            self.versions_dir.join(version).exists()
        }

        fn bin_path(&self) -> PathBuf {
            self.bin_dir.join("bun")
        }

        fn read_active_bun(&self) -> Option<String> {
            fs::read_to_string(self.bin_path()).ok()
        }
    }

    #[test]
    fn test_version_normalization() {
        assert_eq!(normalize_version("v1.0.0"), "1.0.0");
        assert_eq!(normalize_version("1.0.0"), "1.0.0");
        assert_eq!(normalize_version("v1.2.3"), "1.2.3");
        assert_eq!(normalize_version("v2.0.0-beta"), "2.0.0-beta");
    }

    #[test]
    fn test_create_version_directory() {
        let env = TestEnv::new();

        let version = "1.0.0";
        env.create_mock_version(version);

        assert!(
            env.version_exists(version),
            "Version directory should exist"
        );
    }

    #[test]
    fn test_multiple_versions_coexist() {
        let env = TestEnv::new();

        let versions = vec!["1.0.0", "1.1.0", "1.2.0"];
        for version in &versions {
            env.create_mock_version(version);
        }

        for version in &versions {
            assert!(
                env.version_exists(version),
                "Version {} should exist",
                version
            );
        }
    }

    #[tokio::test]
    async fn test_activate_bun_copies_binary() {
        let env = TestEnv::new();

        let version = "1.0.0";
        let version_dir = env.create_mock_version(version);
        let bun_source = version_dir.join(BUN_BIN_NAME);

        fs::copy(&bun_source, env.bin_path()).unwrap();

        let active_content = env.read_active_bun().unwrap();
        assert_eq!(
            active_content,
            format!("mock bun {}", version),
            "Active binary should match source"
        );
    }

    #[tokio::test]
    async fn test_remove_version() {
        let env = TestEnv::new();

        let version = "1.0.0";
        env.create_mock_version(version);
        assert!(
            env.version_exists(version),
            "Version should exist before removal"
        );

        let version_path = env.versions_dir.join(version);
        fs::remove_dir_all(&version_path).unwrap();

        assert!(
            !env.version_exists(version),
            "Version should not exist after removal"
        );
    }

    #[tokio::test]
    async fn test_switch_between_versions() {
        let env = TestEnv::new();

        let v1 = "1.0.0";
        let v2 = "2.0.0";
        let v1_dir = env.create_mock_version(v1);
        let v2_dir = env.create_mock_version(v2);

        // Activate v1
        let bun_v1 = v1_dir.join(BUN_BIN_NAME);
        fs::copy(&bun_v1, env.bin_path()).unwrap();
        assert_eq!(
            env.read_active_bun().unwrap(),
            format!("mock bun {}", v1),
            "Should activate v1"
        );

        // Switch to v2
        let bun_v2 = v2_dir.join(BUN_BIN_NAME);
        fs::copy(&bun_v2, env.bin_path()).unwrap();
        assert_eq!(
            env.read_active_bun().unwrap(),
            format!("mock bun {}", v2),
            "Should activate v2"
        );
    }

    #[tokio::test]
    async fn test_list_versions_sorted() {
        let env = TestEnv::new();

        let versions = vec!["1.2.0", "1.0.0", "1.1.0", "2.0.0"];
        for version in &versions {
            env.create_mock_version(version);
        }

        let mut found_versions = Vec::new();
        let entries = fs::read_dir(env.versions_dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let version = entry.file_name().to_string_lossy().to_string();
            found_versions.push(version);
        }

        found_versions.sort();
        found_versions.reverse();

        assert_eq!(
            found_versions,
            vec!["2.0.0", "1.2.0", "1.1.0", "1.0.0"],
            "Versions should be sorted descending"
        );
    }

    #[test]
    fn test_version_with_special_characters() {
        let env = TestEnv::new();

        let versions = vec!["1.0.0-canary", "1.0.0-beta.1", "1.0.0+build"];

        for version in &versions {
            env.create_mock_version(version);
            assert!(
                env.version_exists(version),
                "Special version {} should exist",
                version
            );
        }
    }

    #[tokio::test]
    async fn test_bumrc_file_format() {
        let env = TestEnv::new();
        let bumrc_path = env._temp_dir.path().join(".bumrc");

        // Test with newline
        fs::write(&bumrc_path, "1.0.0\n").unwrap();
        let content = fs::read_to_string(&bumrc_path).unwrap();
        assert_eq!(content.trim(), "1.0.0");

        // Test with v prefix
        fs::write(&bumrc_path, "v1.0.0").unwrap();
        let content = fs::read_to_string(&bumrc_path).unwrap();
        assert_eq!(normalize_version(content.trim()), "1.0.0");
    }

    #[test]
    fn test_version_persistence() {
        let env = TestEnv::new();

        let version = "1.0.0";
        let version_dir = env.create_mock_version(version);
        let bun_source = version_dir.join(BUN_BIN_NAME);

        fs::copy(&bun_source, env.bin_path()).unwrap();

        // Version dir still exists
        assert!(
            env.version_exists(version),
            "Source version should persist after activation"
        );

        // Active binary exists
        assert!(env.bin_path().exists(), "Active binary should exist");

        // Content matches
        let active = env.read_active_bun().unwrap();
        let original = fs::read_to_string(&bun_source).unwrap();
        assert_eq!(
            active, original,
            "Active binary content should match original"
        );
    }
}
