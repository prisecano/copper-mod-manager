use clap::{Parser, Subcommand};

/// Tool to manage Minecraft mods for Fabric + Modrinth
#[derive(Debug, Parser)]
#[command(name = "cmm")]
#[command(about = "Tool to manage Minecraft mods for Fabric + Modrinth", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a mod
    #[command(arg_required_else_help = true)]
    Add {
        /// The Minecraft version that supports the mod
        minecraft_version: String,
        /// The mod slug or ID from Modrinth; https://modrinth.com/mod/SLUG
        id_or_slug: String,
    },
    /// Remove a mod
    #[command(arg_required_else_help = true)]
    Rm {
        /// The Minecraft mod file to remove; Use command 'list' for lookup and copy-and-paste the one you want to remove
        minecraft_file: String,
    },
    /// Show mods in mods directory
    #[command()]
    List,
    /// Check if mods have a new version, and option to download them
    #[command(arg_required_else_help = true)]
    Latest {
        /// The Minecraft version to check if mods are up-to-date
        minecraft_version: String,
    },
    /// Check if mods are supported for the next or older Minecraft version, and option to download them
    #[command(arg_required_else_help = true)]
    Support {
        /// The Minecraft version to check if mods are supported
        minecraft_version: String,
    },
}
