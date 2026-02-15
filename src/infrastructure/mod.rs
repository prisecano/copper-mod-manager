use crate::domain::{contract::file_system::IFileSystem, entities::minecraft_mod::MinecraftMod};

pub(crate) mod file_system;
pub(crate) mod modrinth_api;

pub(crate) struct InMemFileSystem;
pub(crate) struct FileSystem;

impl IFileSystem for InMemFileSystem {
    fn hash_file(&self) -> String {
        println!("In mem");

        "lol".to_string()
    }

    fn remove_mc_mod_by_file_name(
        &self,
        mc_mod_file_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("{mc_mod_file_name}");
        todo!()
    }

    fn get_mod_file_paths(&self) -> Vec<MinecraftMod> {
        println!("In mem");

        vec![]
    }
}
