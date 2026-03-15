use colored::Colorize;

pub(crate) fn opening_message(mc_version: &str) {
    println!(
        "\r\n{} {}",
        "Checking if current mods has versions on Modrinth that support".bright_cyan(),
        mc_version.bright_blue()
    );
}

pub(crate) fn mc_mods_already_supported_message(mc_version: &str) {
    println!(
        "\r\n{} {}!",
        "Mod(s) already supports".bright_green(),
        mc_version.bright_green()
    );
}

pub(crate) fn update_mods_choice() -> Option<char> {
    println!("\r\nUpdate mod(s)? [y]es, [n]o");

    let mut update_choice = String::new();
    std::io::stdin()
        .read_line(&mut update_choice)
        .unwrap_or_default();

    update_choice.trim().chars().next()
}
