use colored::Colorize;

use crate::domain::minecraft_mod::MinecraftModVersionDiff;

pub(crate) fn select_opening_message(mc_mod_version_diff: &MinecraftModVersionDiff) {
    println!(
        "\r\n{} -> {}",
        mc_mod_version_diff.file_name.bright_blue(),
        mc_mod_version_diff
            .minecraft_mod_new_version
            .file_name
            .bright_blue()
    );
    println!(
        "{}\r\n{}",
        "Changelog:".bright_cyan(),
        mc_mod_version_diff
            .minecraft_mod_new_version
            .changelog
            .on_black()
    );
}

pub(crate) fn select_choice() -> Option<char> {
    println!("\r\n{} [y]es, [n]o:", "Update this mod?".bright_cyan());

    let mut update_choice = String::new();
    std::io::stdin()
        .read_line(&mut update_choice)
        .unwrap_or_default();

    update_choice.trim().chars().next()
}
