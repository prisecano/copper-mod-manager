use colored::Colorize;

use crate::{
    adapter, contract::add::AddMinecraftMod, entity::MinecraftMod, service::MinecraftModsService,
    utils,
};

impl AddMinecraftMod for MinecraftModsService {
    async fn add(&mut self, mc_version: &str, id_or_slug: &str) {
        let mut mc_mod_download_url = MinecraftMod::new();
        adapter::lists_projects_versions_to_new_mc_mod_download_url(
            mc_version,
            id_or_slug,
            &mut mc_mod_download_url,
        )
        .await;

        println!("\r\n{}", "Found!".bright_green());

        println!(
            "\r\n{} {id_or_slug}{}",
            "Downloading".bright_cyan(),
            "...".bright_cyan()
        );

        match utils::add_mc_mod(&mc_mod_download_url).await {
            Ok(_) => println!("\r\n{}", "Download successfull!".bright_green()),
            Err(_) => println!("\r\n{}", "Download failed...".bright_red()),
        };
    }
}
