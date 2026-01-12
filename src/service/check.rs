use colored::Colorize;

use super::super::modrinth_api;
use super::check_mods_compatible;
use super::hash_mc_mods;

pub async fn check(minecraft_version: &String) {
    println!(
        "\r\n{} {}",
        "Checking if mods are available for minecraft version".bright_cyan(),
        minecraft_version.bright_blue()
    );

    let game_versions = &vec![minecraft_version];
    let loader = &"fabric".to_string();
    let loaders = &vec![loader];

    println!(
        "\r\n{}",
        "Obtaining hashes for Modrinth lookup:".bright_cyan()
    );
    let modpack = hash_mc_mods();
    let mods_hashes: &Vec<&String> = &modpack.iter().map(|h| &h.1).collect();

    println!("\r\n{}", "Searching mods on Modrinth...".bright_cyan());

    let body_text =
        modrinth_api::latest_version_of_multiple_project(mods_hashes, loaders, game_versions)
            .await
            .unwrap_or_default();

    println!("\r\n{}", "Done!".bright_green());

    check_mods_compatible(modpack, minecraft_version, body_text);
}
