use clap::Parser;

use cli::{Cli, Commands};

mod adapter;
mod cli;
mod modrinth_api;
mod service;

pub async fn run() {
    let args = Cli::parse();

    match args.command {
        Commands::Add {
            minecraft_version,
            id_or_slug,
        } => service::add(&minecraft_version, &id_or_slug).await,
        Commands::Rm { minecraft_file } => service::rm(&minecraft_file),
        Commands::List => service::list(),
        Commands::Latest { minecraft_version } => service::latest(&minecraft_version).await,
        Commands::Support { minecraft_version } => service::support(&minecraft_version).await,
    }
}
