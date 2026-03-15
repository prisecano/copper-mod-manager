use colored::Colorize;

pub(crate) fn update_mods_ui(amount_latest_mods: usize) -> Option<char> {
    if amount_latest_mods == 0 {
        println!("\r\n{}", "Mods are up-to-date!".bright_green());
        return Some('n');
    } else {
        println!(
            "\r\n{}",
            format!("There are {amount_latest_mods} mod(s) with newer version!").bright_yellow()
        );
        println!("\r\nUpdate [a]ll, [s]elect, [n]one?");

        let mut update_choice = String::new();
        std::io::stdin()
            .read_line(&mut update_choice)
            .unwrap_or_default();

        update_choice.trim().chars().next()
    }
}
