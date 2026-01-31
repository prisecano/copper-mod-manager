use crate::{domain::contract::latest::LatestMinecraftMods, service::MinecraftModsService};

pub(crate) async fn latest_view(mc_version: &str) {
    let mut mc_mods_service = MinecraftModsService::default();
    mc_mods_service.latest(mc_version).await;
}
