use crate::{
    domain::contract::rm::RemoveMinecraftMod, infrastructure::file_system,
    service::MinecraftModsService,
};

impl RemoveMinecraftMod for MinecraftModsService {
    fn rm(&self, mc_mod_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        file_system::remove_mc_mod_by_file_name(mc_mod_file_name)?;

        Ok(())
    }
}
