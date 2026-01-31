use crate::domain::entities::minecraft_mod::MinecraftMod;

pub(crate) mod add;
pub(crate) mod latest;
pub(crate) mod list;
pub(crate) mod rm;
pub(crate) mod support;

pub(crate) struct MinecraftModsService {
    pub(crate) mc_mods: Vec<MinecraftMod>,
}

impl Default for MinecraftModsService {
    fn default() -> Self {
        Self {
            mc_mods: Default::default(),
        }
    }
}
