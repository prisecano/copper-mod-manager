use serde_json::Value;

use crate::modrinth_api;
use crate::service::entities::MinecraftMod;

pub(crate) async fn lists_projects_versions_to_new_mc_mod(
    mc_version: &str,
    id_or_slug: &str,
    mc_mod: &mut MinecraftMod,
) {
    let ids_or_slugs = &vec![id_or_slug];
    let game_versions = &vec![mc_version];
    let loaders = &vec!["fabric"];

    let body = modrinth_api::lists_projects_versions(ids_or_slugs, loaders, game_versions)
        .await
        .unwrap_or_default();

    if let Some(versions) = body.as_array() {
        let files = versions[0].get("files").and_then(Value::as_array).unwrap();
        let file_download_url = files[0].get("url").and_then(Value::as_str).unwrap();
        let file_name = files[0].get("filename").and_then(Value::as_str).unwrap();

        mc_mod.file_name = file_name.to_owned();
        mc_mod.download_url = file_download_url.to_owned();
    }
}
