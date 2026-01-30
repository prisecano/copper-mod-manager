pub(crate) trait SupportMinecraftMods {
    async fn support(&mut self, mc_version: &str);
}
