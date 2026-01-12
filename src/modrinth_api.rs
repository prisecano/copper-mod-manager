use std::collections::HashMap;

const MODRINTH_URL: &str = "https://api.modrinth.com/v2/";

pub async fn latest_version_of_multiple_project(
    hashes: &Vec<&String>,
    loaders: &Vec<&String>,
    game_versions: &Vec<&String>,
) -> Result<String, reqwest::Error> {
    let mut payload = HashMap::new();
    payload.insert("hashes", hashes);
    payload.insert("loaders", loaders);
    payload.insert("game_versions", game_versions);

    let client = reqwest::Client::new();
    let res = client
        .post(MODRINTH_URL.to_owned() + "version_files/update")
        .json(&payload)
        .send()
        .await?;

    let body_text = res.text().await?;
    //println!("\r\nbody:\r\n{}", body_json.to_string());

    Ok(body_text)
}
