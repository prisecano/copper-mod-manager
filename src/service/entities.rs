use colored::Colorize;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub type MinecraftMods = Vec<MinecraftMod>;

pub(crate) struct MinecraftModVersionUpdate {
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
    pub fn new() -> Self {
        Self {
            file_name: String::new(),
            file_path: PathBuf::new(),
            file_hash: String::new(),
            changelog: String::new(),
            download_url: String::new(),
        }
    }

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

    pub fn hash_file_sha1(&mut self) {
        let file = File::open(self.file_path.as_path()).unwrap();
        let mut reader = BufReader::new(file);

        let mut hasher = Sha1::new();

        let mut buffer = [0u8; 8192];
        loop {
            let count = reader.read(&mut buffer).unwrap_or_default();
            if count == 0 {
                break;
            }

            hasher.update(&buffer[..count]);
        }

        let hash = hasher.finalize();

        let mut buffer: [u8; 40] = [0; 40];

        self.file_hash =
            String::from(base16ct::lower::encode_str(&hash, &mut buffer).unwrap_or_default());

        println!("{} -> {}", self.file_name, self.file_hash.yellow());
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
