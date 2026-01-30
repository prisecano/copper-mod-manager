use crate::{contract::list::ListMinecraftModsFileName, service::MinecraftModsService, utils};

impl ListMinecraftModsFileName for MinecraftModsService {
    fn list(&mut self) {
        utils::get_mod_file_paths(self);
    }
}
