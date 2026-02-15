use colored::Colorize;

use crate::{
    domain::contract::minecraft_mods_service::IMinecraftModsService,
    infrastructure::InMemFileSystem, service::MinecraftModsService,
};

pub(crate) fn rm_view(mc_mod_file_name: &str) {
    println!(
        "{} {}...",
        "Removing".bright_cyan(),
        mc_mod_file_name.bright_blue()
    );

    let mut mc_mods_service = MinecraftModsService::new(InMemFileSystem {});

    match mc_mods_service.rm(mc_mod_file_name) {
        Ok(_) => {
            mc_mods_service.list();
            println!("{}", mc_mod_file_name.red())
        }
        Err(err) => println!("\r\n{err}"),
    }
}
