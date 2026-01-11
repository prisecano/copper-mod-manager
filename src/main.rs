use std::env;

mod modrinth_api;
mod service;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let game_versions = [&args[1]];
    let loaders = [&args[2]];

    println!("Current minecraft version is {}", game_versions[0]);
    println!("Current loader is {}", loaders[0]);

    let mods_hashes = service::hash_mc_mods();

    modrinth_api::latest_version_of_a_project(&mods_hashes[3], &loaders, &game_versions)
        .await
        .unwrap_or_default();
}
