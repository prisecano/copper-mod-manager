use clap::Parser;

use cli::{Cli, Commands};

mod cli;
mod domain;
mod infrastructure;
mod presentation;
mod service;

pub async fn run() {
    let args = Cli::parse();

    match args.command {
        Commands::Add {
            minecraft_version,
            id_or_slug,
        } => service::minecraft_mods::add(&minecraft_version, &id_or_slug).await,
        Commands::Rm {
            minecraft_file: minecraft_mod_file_name,
        } => service::minecraft_mods::rm(&minecraft_mod_file_name),
        Commands::List => service::minecraft_mods::list(),
        Commands::Latest { minecraft_version } => {
            service::minecraft_mods::latest(&minecraft_version).await
        }
        Commands::Support { minecraft_version } => {
            service::minecraft_mods::support(&minecraft_version).await
        }
    }
}
