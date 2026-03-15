use colored::Colorize;

pub(crate) fn opening_message(id_or_slug: &str) {
    println!(
        "\r\n{} {id_or_slug} {}",
        "Searching".bright_cyan(),
        "on Modrinth...".bright_cyan()
    );
}
