pub(crate) trait AddMinecraftMod {
    async fn add(&mut self, mc_version: &str, id_or_slug: &str);
}
