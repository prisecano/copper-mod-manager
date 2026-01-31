use std::collections::HashMap;

use serde_json::Value;

const MODRINTH_URL: &str = "https://api.modrinth.com/v2/";

pub(crate) async fn latest_version_of_multiple_project(
    hashes: &Vec<&str>,
    loaders: &Vec<&str>,
    game_versions: &Vec<&str>,
) -> Result<Value, reqwest::Error> {
    let mut payload = HashMap::new();
    payload.insert("hashes", hashes);
    payload.insert("loaders", loaders);
    payload.insert("game_versions", game_versions);

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{MODRINTH_URL}version_files/update"))
        .json(&payload)
        .send()
        .await?;

    let body_text = response.text().await?;
    let body_json = serde_json::from_str(&body_text).unwrap_or_default();

    Ok(body_json)
}

pub(crate) async fn lists_projects_versions(
    id_or_slug: &Vec<&str>,
    loaders: &Vec<&str>,
    game_versions: &Vec<&str>,
) -> Result<Value, reqwest::Error> {
    let include_changelog = false;

    let url = format!(
        "{MODRINTH_URL}project/{}/version?loaders={:?}&game_versions={:?}&include_changelog={include_changelog}",
        id_or_slug[0], loaders, game_versions
    );
    println!("{url}");

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        panic!("API request failed with {}", response.status());
    }

    let body_text = response.text().await?;
    let body_json = serde_json::from_str(&body_text).unwrap_or_default();

    Ok(body_json)
}
