use crate::domain::contract::file_system::IFileSystem;

pub(crate) mod minecraft_mods;

pub(crate) struct MinecraftModsService<FileSystem: IFileSystem> {
    file_system: FileSystem,
}

impl<FileSystem: IFileSystem> MinecraftModsService<FileSystem> {
    pub(crate) fn new(file_system: FileSystem) -> Self {
        Self { file_system }
    }
}
