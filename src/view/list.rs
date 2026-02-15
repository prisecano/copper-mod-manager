use colored::Colorize;

use crate::{
    domain::contract::minecraft_mods_service::IMinecraftModsService,
    infrastructure::InMemFileSystem, service::MinecraftModsService,
};

pub(crate) fn list_view() {
    println!(
        "{}...",
        "Displaying Minecraft mods in the mods directory".bright_cyan()
    );

    let mut mc_mods_service = MinecraftModsService::new(InMemFileSystem {});
    let mc_mods = mc_mods_service.list();

    mc_mods
        .iter()
        .for_each(|mc_mod| println!("{}", mc_mod.file_name.on_green()));
}
