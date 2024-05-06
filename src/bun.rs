use anyhow::Result;
use serde::Deserialize;
use tokio::process::Command;

const BUN_GITHUB_TAGS_URL: &str = "https://api.github.com/repos/oven-sh/bun/tags";

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
