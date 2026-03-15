use colored::Colorize;

pub(crate) fn opening_message(mc_mod_file_name: &str) {
    println!(
        "{} {}...",
        "Removing".bright_cyan(),
        mc_mod_file_name.bright_blue()
    );
}
