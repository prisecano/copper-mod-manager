use clap::Parser;

use cli::{Cli, Commands};

mod adapter;
mod cli;
mod modrinth_api;
mod service;

pub async fn run() {
    let args = Cli::parse();

    match args.command {
        Commands::Support { minecraft_version } => service::support(&minecraft_version).await,
        Commands::Update { minecraft_version } => {
            println!("Update mods for Minecraft version {minecraft_version}");
        }
        Commands::Add {
            minecraft_version,
            id_or_slug,
        } => service::add(&minecraft_version, &id_or_slug).await,
        Commands::List => service::list(),
        Commands::Rm { minecraft_file } => service::rm(&minecraft_file),
        Commands::Latest { minecraft_version } => service::latest(&minecraft_version).await, // tell user there are n updates or no updates
    }
}
