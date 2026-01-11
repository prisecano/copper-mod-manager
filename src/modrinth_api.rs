use serde_json::Value;
use std::{collections::HashMap, str::FromStr};

const MODRINTH_URL: &str = "https://api.modrinth.com/v2/";

pub async fn latest_version_of_a_project(
    hash: &str,
    loaders: &[&String; 1],
    game_versions: &[&String; 1],
) -> Result<(), reqwest::Error> {
    let mut payload = HashMap::new();
    payload.insert("loaders", loaders);
    payload.insert("game_versions", game_versions);

    let client = reqwest::Client::new();
    let res = client
        .post(MODRINTH_URL.to_owned() + "version_file/" + hash + "/update")
        .json(&payload)
        .send()
        .await?;

    let body_text = res.text().await?;
    let body_json: Value = serde_json::from_str(body_text.as_str()).unwrap_or_default();

    let body_game_versions: &str = body_json["game_versions"][0].as_str().unwrap_or_default();

    let mut compatibility: String = String::from_str(hash).unwrap_or_default();
    println!(
        "Compatibility check for Minecraft version: {}",
        body_game_versions
    );
    if body_game_versions == game_versions[0] {
        compatibility.push_str(" -> ✔");
    } else {
        compatibility.push_str(" -> ❌");
    };
    println!("{compatibility}\r\n");

    println!("body:\r\n{}", body_json.to_string());

    Ok(())
}
