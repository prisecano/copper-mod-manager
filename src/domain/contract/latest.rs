pub(crate) trait LatestMinecraftMods {
    async fn latest(&mut self, mc_version: &str);
}
