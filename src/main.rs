use clap::{Parser, Subcommand};
mod modrinth_api;
mod service;

/// Tool to manage minecraft mods for Fabric + Modrinth
#[derive(Debug, Parser)]
#[command(name = "cmm")]
#[command(about = "Tool to manage minecraft mods for Fabric + Modrinth", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Check if mods are available for a given minecraft version
    #[command(arg_required_else_help = true)]
    Check {
        /// The minecraft version to check mods
        minecraft_version: String,
    },
    /// Update mods to the latest version for a given minecraft version
    #[command(arg_required_else_help = true)]
    Update {
        /// The minecraft version to update mods
        minecraft_version: String,
    },
}

#[cfg(windows)]
fn enable_ansi_support() {
    colored::control::set_virtual_terminal(true).unwrap();
}

#[tokio::main]
async fn main() {
    enable_ansi_support();

    let args = Cli::parse();

    match args.command {
        Commands::Check { minecraft_version } => service::check::check(&minecraft_version).await,
        Commands::Update { minecraft_version } => {
            println!("Update mods for Minecraft version {minecraft_version}");
        }
    }
}
