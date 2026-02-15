use colored::Colorize;

use crate::{
    adapter,
    domain::{
        contract::minecraft_mods_service::IMinecraftModsService,
        entities::minecraft_mod::MinecraftMod,
    },
    service::{IFileSystem, MinecraftModsService},
    utils,
};

impl<FileSystem> IMinecraftModsService for MinecraftModsService<FileSystem>
where
    FileSystem: IFileSystem,
{
    async fn add(&mut self, mc_version: &str, id_or_slug: &str) {
        let mc_mod_download_url =
            adapter::lists_projects_versions_to_new_mc_mod_download_url(mc_version, id_or_slug)
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

    async fn latest(&mut self, mc_version: &str) {
        let mut mc_mods = self.file_system.get_mod_file_paths();
        utils::parallise_hashing_mc_mods(&mut mc_mods);

        let body = adapter::get_latest_version_of_multiple_project(mc_version, &mut mc_mods).await;

        let mc_mods_version_diff =
            utils::get_latest_mods_version_diff(&body, mc_version, &mut mc_mods);

        let n = mc_mods_version_diff.len();
        if n == 0 {
            println!("\r\n{}", "Mods are up-to-date!".bright_green())
        } else {
            println!(
                "\r\n{}",
                format!("There are {n} mod(s) with newer version!").bright_yellow()
            );
            println!("\r\nUpdate [a]ll, [s]elect, [n]one?");

            let mut update_choice = String::new();
            std::io::stdin()
                .read_line(&mut update_choice)
                .unwrap_or_default();

            match update_choice.trim().chars().next() {
                Some('a') => utils::update_all(&mc_mods_version_diff).await,
                Some('s') => utils::update_selective(&mc_mods_version_diff).await,
                Some('n') => return,
                _ => println!("Invalid input"),
            }
        }
    }

    fn list(&mut self) -> Vec<MinecraftMod> {
        self.file_system.get_mod_file_paths()
    }

    fn rm(&mut self, mc_mod_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.file_system
            .remove_mc_mod_by_file_name(mc_mod_file_name)?;

        Ok(())
    }

    async fn support(&mut self, mc_version: &str) {
        let mut mc_mods = self.file_system.get_mod_file_paths();
        utils::parallise_hashing_mc_mods(&mut mc_mods);

        let body = adapter::get_latest_version_of_multiple_project(mc_version, &mut mc_mods).await;

        let is_supported = utils::check_support_mc_mods(&mc_mods, mc_version, &body);

        if is_supported {
            utils::update_mods_to_support_a_mc_version_ui(&body, mc_version, &mut mc_mods).await;
        }
    }
}
