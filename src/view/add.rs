use colored::Colorize;

use crate::{domain::contract::add::AddMinecraftMod, service::MinecraftModsService};

pub(crate) async fn add_view(mc_version: &str, id_or_slug: &str) {
    println!(
        "\r\n{} {id_or_slug} {}",
        "Searching".bright_cyan(),
        "on Modrinth...".bright_cyan()
    );

    let mut mc_mods_service = MinecraftModsService::default();
    mc_mods_service.add(mc_version, id_or_slug).await;
}
