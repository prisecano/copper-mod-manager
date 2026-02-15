use std::fs::File;
use std::{error::Error, fs, io::copy};

use walkdir::{DirEntry, WalkDir};

use crate::domain::contract::file_system::IFileSystem;
use crate::domain::entities::minecraft_mod::MinecraftMod;
use crate::infrastructure::FileSystem;

impl IFileSystem for FileSystem {
    fn hash_file(&self) -> String {
        todo!()
    }

    fn remove_mc_mod_by_file_name(
        &self,
        mc_mod_file_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        fs::remove_file("mods/".to_string() + mc_mod_file_name)?;
        Ok(())
    }

    fn get_mod_file_paths(&self) -> Vec<MinecraftMod> {
        let mc_mods_in_mods_dir = WalkDir::new("mods")
            .into_iter()
            .filter_entry(|e| should_include(e))
            .filter_map(|e| e.ok());

        mc_mods_in_mods_dir
            .into_iter()
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| MinecraftMod::new_mc_mod_by_path(entry.into_path()))
            .collect()
    }
}

pub(crate) async fn copy_downloaded_mc_mod_to_file_by_file_name(
    file_name: &str,
    mut response: reqwest::Response,
) -> Result<(), Box<dyn Error + 'static>> {
    let mut dest = File::create(format!("mods/{file_name}")).unwrap();
    while let Some(chunk) = response.chunk().await? {
        copy(&mut chunk.as_ref(), &mut dest).unwrap();
    }

    Ok(())
}

pub(crate) fn should_include(entry: &DirEntry) -> bool {
    if entry.file_type().is_dir() {
        return true;
    }

    match entry.path().extension().and_then(|e| e.to_str()) {
        Some("jar") => true,
        _ => false,
    }
}
