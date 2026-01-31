pub(crate) trait RemoveMinecraftMod {
    fn rm(&self, mc_mod_file_name: &str) -> Result<(), Box<dyn std::error::Error>>;
}
