use colored::Colorize;

use crate::{
    domain::contract::minecraft_mods_service::IMinecraftModsService,
    infrastructure::InMemFileSystem, service::MinecraftModsService,
};

pub(crate) async fn support_view(mc_version: &str) {
    println!(
        "\r\n{} {}",
        "Checking if current mods has versions on Modrinth that support".bright_cyan(),
        mc_version.bright_blue()
    );

    let mut mc_mods_service = MinecraftModsService::new(InMemFileSystem {});
    mc_mods_service.support(mc_version).await;
}
