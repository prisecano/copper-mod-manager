use crate::domain::entities::minecraft_mod::MinecraftMod;

pub(crate) trait IMinecraftModsService {
    async fn add(&mut self, mc_version: &str, id_or_slug: &str);
    async fn latest(&mut self, mc_version: &str);
    fn list(&mut self) -> Vec<MinecraftMod>;
    fn rm(&mut self, mc_mod_file_name: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn support(&mut self, mc_version: &str);
}
