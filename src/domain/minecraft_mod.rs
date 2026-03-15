use std::path::PathBuf;

pub(crate) mod rules;

pub(crate) struct MinecraftModVersionDiff {
    pub(crate) file_name: String,
    pub(crate) minecraft_mod_new_version: MinecraftMod,
}

#[derive(Clone)]
pub(crate) struct MinecraftMod {
    pub(crate) file_name: String,
    pub(crate) file_path: PathBuf,
    pub(crate) file_hash: String,
    pub(crate) changelog: String,
    pub(crate) download_url: String,
}

impl MinecraftMod {
    pub fn new_mc_mod_by_path(file_path: PathBuf) -> Self {
        Self {
            file_name: String::from(
                file_path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            ),
            file_path,
            file_hash: String::new(),
            changelog: String::new(),
            download_url: String::new(),
        }
    }
}

impl Default for MinecraftMod {
    fn default() -> Self {
        Self {
            file_name: Default::default(),
            file_path: Default::default(),
            file_hash: Default::default(),
            changelog: Default::default(),
            download_url: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minecraft_mod_path() {
        let path: PathBuf = PathBuf::from("mods/foo.jar");

        let mc_mod: MinecraftMod = MinecraftMod::new_mc_mod_by_path(path);

        assert_eq!(mc_mod.file_path, PathBuf::from("mods/foo.jar"));
        assert_eq!(mc_mod.file_name, PathBuf::from("foo.jar"));
    }
}
