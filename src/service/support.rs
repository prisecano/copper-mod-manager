use crate::{
    domain::contract::support::SupportMinecraftMods, service::MinecraftModsService, utils,
};

impl SupportMinecraftMods for MinecraftModsService {
    async fn support(&mut self, mc_version: &str) {
        let body = utils::get_latest_version_of_multiple_project(mc_version, self).await;

        let is_supported = utils::check_support_mc_mods(self, mc_version, &body);

        if is_supported {
            utils::update_mods_to_support_a_mc_version_ui(&body, mc_version, self).await;
        }
    }
}
