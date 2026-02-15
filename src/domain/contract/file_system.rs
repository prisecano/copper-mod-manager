use crate::domain::entities::minecraft_mod::MinecraftMod;

pub(crate) trait IFileSystem {
    fn hash_file(&self) -> String;
    fn remove_mc_mod_by_file_name(
        &self,
        mc_mod_file_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn get_mod_file_paths(&self) -> Vec<MinecraftMod>;
}
