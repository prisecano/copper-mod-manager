use crate::{domain::contract::latest::LatestMinecraftMods, service::MinecraftModsService};

use super::super::utils;

impl LatestMinecraftMods for MinecraftModsService {
    async fn latest(&mut self, mc_version: &str) {
        let body = utils::get_latest_version_of_multiple_project(mc_version, self).await;
        utils::check_latest_mc_mods(&body, mc_version, self).await;
    }
}
