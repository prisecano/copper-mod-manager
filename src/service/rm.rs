use crate::{contract::rm::RemoveMinecraftMod, service::MinecraftModsService, utils};

impl RemoveMinecraftMod for MinecraftModsService {
    fn rm(&self, mc_mod_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        utils::remove_mc_mod_by_file_name(mc_mod_file_name)?;

        Ok(())
    }
}
