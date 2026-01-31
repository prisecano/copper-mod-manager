use clap::Parser;

use cli::{Cli, Commands};

mod adapter;
mod cli;
mod domain;
mod infrastructure;
mod service;
mod utils;
mod view;

pub async fn run() {
    let args = Cli::parse();

    match args.command {
        Commands::Add {
            minecraft_version,
            id_or_slug,
        } => view::add::add_view(&minecraft_version, &id_or_slug).await,
        Commands::Rm {
            minecraft_file: minecraft_mod_file_name,
        } => view::rm::rm_view(&minecraft_mod_file_name),
        Commands::List => view::list::list_view(),
        Commands::Latest { minecraft_version } => {
            view::latest::latest_view(&minecraft_version).await
        }
        Commands::Support { minecraft_version } => {
            view::support::support_view(&minecraft_version).await
        }
    }
}
