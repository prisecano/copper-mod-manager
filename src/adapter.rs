use colored::Colorize;
use serde_json::Value;

use crate::{domain::entities::minecraft_mod::MinecraftMod, infrastructure::modrinth_api};

pub(crate) async fn lists_projects_versions_to_new_mc_mod_download_url(
    mc_version: &str,
    id_or_slug: &str,
) -> MinecraftMod {
    let ids_or_slugs = &vec![id_or_slug];
    let game_versions = &vec![mc_version];
    let loaders = &vec!["fabric"];

    let body = modrinth_api::lists_projects_versions(ids_or_slugs, loaders, game_versions)
        .await
        .unwrap_or_default();

    let mut mc_mod = MinecraftMod::default();

    if let Some(versions) = body.as_array() {
        let files = versions[0].get("files").and_then(Value::as_array).unwrap();
        let file_download_url = files[0].get("url").and_then(Value::as_str).unwrap();
        let file_name = files[0].get("filename").and_then(Value::as_str).unwrap();

        mc_mod.file_name = file_name.to_owned();
        mc_mod.download_url = file_download_url.to_owned();
    }

    mc_mod
}

pub(crate) async fn get_latest_version_of_multiple_project(
    mc_version: &str,
    mc_mods: &mut Vec<MinecraftMod>,
) -> Value {
    let game_versions = &vec![mc_version];
    let loaders = &vec!["fabric"];

    let mods_hashes: &Vec<&str> = &mc_mods
        .iter()
        .map(|mc_mod| mc_mod.file_hash.as_str())
        .collect();

    println!("\r\n{}", "Searching mods on Modrinth...".bright_cyan());

    let body =
        modrinth_api::latest_version_of_multiple_project(mods_hashes, loaders, game_versions)
            .await
            .unwrap_or_default();

    println!("\r\n{}", "Done!".bright_green());
    body
}
