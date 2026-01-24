use entities::MinecraftMods;

use crate::{adapter, service::entities::MinecraftMod};

use colored::Colorize;

pub mod entities;
mod utils;

pub(crate) async fn support(mc_version: &str) {
    let mut mc_mods: MinecraftMods = Vec::new();
    println!(
        "\r\n{} {}",
        "Checking if mods are supported for minecraft version".bright_cyan(),
        mc_version.bright_blue()
    );

    let body_text = utils::get_latest_version_of_multiple_project(mc_version, &mut mc_mods).await;

    utils::check_support_mc_mods(mc_mods, mc_version, &body_text);
}

pub(crate) fn list() {
    let mut mc_mods: MinecraftMods = Vec::new();
    utils::get_mod_file_paths(&mut mc_mods);

    mc_mods
        .iter()
        .for_each(|mc_mod| println!("{}", mc_mod.file_name.green()));
}

pub(crate) fn rm(mc_mod_file: &str) {
    utils::remove_mc_mod_by_mc_mod_file_name(mc_mod_file).unwrap_or_else(|err| println!("{}", err));
    list();
    println!("{}", mc_mod_file.red())
}

pub(crate) async fn add(mc_version: &str, id_or_slug: &str) {
    println!(
        "\r\n{} {id_or_slug} {}",
        "Searching".bright_cyan(),
        "on Modrinth...".bright_cyan()
    );

    let mut mc_mod = MinecraftMod::new();
    adapter::lists_projects_versions_to_new_mc_mod(mc_version, id_or_slug, &mut mc_mod).await;

    println!("\r\n{}", "Found!".bright_green());

    println!(
        "\r\n{} {id_or_slug}{}",
        "Downloading".bright_cyan(),
        "...".bright_cyan()
    );

    match utils::add_mc_mod(&mc_mod).await {
        Ok(_) => println!("\r\n{}", "Download successfull!".bright_green()),
        Err(_) => println!("\r\n{}", "Download failed...".bright_red()),
    };
}

pub(crate) async fn latest(mc_version: &str) {
    let mut mc_mods: MinecraftMods = Vec::new();
    let body_text = utils::get_latest_version_of_multiple_project(mc_version, &mut mc_mods).await;
    utils::check_latest_mc_mods(&body_text, mc_version, &mut mc_mods).await;
}
