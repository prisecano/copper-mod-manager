use colored::Colorize;
use serde_json::Value;

use crate::domain::minecraft_mod::MinecraftMod;

pub(crate) fn mc_mods_has_a_version_that_is_supported(
    mc_mods: &Vec<MinecraftMod>,
    mc_version: &str,
    body: &Value,
) -> bool {
    println!(
        "\r\n{} {}",
        "Mod(s) support check for".bright_cyan(),
        mc_version.bright_blue()
    );

    let Some(projects) = body.as_object() else {
        return false;
    };
    let supported_hashes: Vec<&String> = projects.keys().collect();

    let yes = "O".green();
    let no = "X".red();

    mc_mods.iter().for_each(|mc_mod| {
        let is_available = match supported_hashes.contains(&&mc_mod.file_hash) {
            true => &yes,
            false => &no,
        };
        println!(
            "{} -> {} {}",
            mc_mod.file_hash.yellow(),
            is_available,
            mc_mod.file_name
        )
    });

    let total_local_mods = mc_mods.len();
    let total_supported_mods = projects.len();

    if total_local_mods != total_supported_mods {
        println!(
            "\r\n{} {}/{} {}",
            "Only".bright_cyan(),
            total_supported_mods.to_string().bright_blue(),
            total_local_mods.to_string().bright_blue(),
            "supported...".bright_cyan()
        );

        return false;
    }

    println!(
        "\r\n{} {}",
        "All mods has versions that support".bright_cyan(),
        mc_version.bright_blue()
    );

    true
}

pub(crate) fn mc_mod_is_already_supported(old_file_name: &String, new_file_name: &str) -> bool {
    *old_file_name == new_file_name
}
