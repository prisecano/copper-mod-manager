use crate::{
    domain::contract::minecraft_mods_service::IMinecraftModsService,
    infrastructure::InMemFileSystem, service::MinecraftModsService,
};

pub(crate) async fn latest_view(mc_version: &str) {
    let mut mc_mods_service = MinecraftModsService::new(InMemFileSystem {});
    mc_mods_service.latest(mc_version).await;
}
