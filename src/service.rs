use super::modrinth_api;
use colored::Colorize;
use rayon::prelude::*;
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

struct MinecraftMod {
    file_name: String,
    file_hash: String,
}

impl MinecraftMod {
    fn new(file_name: String, file_hash: String) -> Self {
        Self {
            file_name,
            file_hash,
        }
    }
    // TODO move hash file fn in here, par from outside and call this method
}

pub async fn check(minecraft_version: &String) {
    println!(
        "\r\n{} {}",
        "Checking if mods are supported for minecraft version".bright_cyan(),
        minecraft_version.bright_blue()
    );

    let game_versions = &vec![minecraft_version];
    let loader = String::from("fabric");
    let loaders = &vec![&loader];

    println!(
        "\r\n{}",
        "Obtaining hashes for Modrinth lookup:".bright_cyan()
    );
    let modpack: Vec<MinecraftMod> = hash_mc_mods();
    let mods_hashes: &Vec<&String> = &modpack.iter().map(|mc_mod| &mc_mod.file_hash).collect();

    println!("\r\n{}", "Searching mods on Modrinth...".bright_cyan());

    let body_text =
        modrinth_api::latest_version_of_multiple_project(mods_hashes, loaders, game_versions)
            .await
            .unwrap_or_default();

    println!("\r\n{}", "Done!".bright_green());

    check_mods_support(modpack, minecraft_version, &body_text);
}

fn hash_mc_mods() -> Vec<MinecraftMod> {
    let mut mc_mods = Vec::new();

    get_mod_file_paths(&mut mc_mods);

    hash_mods(mc_mods)
}

fn get_mod_file_paths(mc_mods: &mut Vec<PathBuf>) {
    for entry in WalkDir::new("mods")
        .into_iter()
        .filter_entry(|e| should_include(e))
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            mc_mods.push(entry.path().to_path_buf());
        }
    }
}

fn hash_mods(mc_mods: Vec<PathBuf>) -> Vec<MinecraftMod> {
    mc_mods
        .par_iter()
        .map(|path| hash_file_sha1(path.as_path()))
        .collect()
}

fn should_include(entry: &DirEntry) -> bool {
    if entry.file_type().is_dir() {
        return true;
    }

    // file extensions filter
    match entry.path().extension().and_then(|e| e.to_str()) {
        Some("jar") => true,
        _ => false,
    }
}

fn hash_file_sha1(path: &Path) -> MinecraftMod {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut hasher = Sha1::new();

    let mut buffer = [0u8; 8192];
    loop {
        let count = reader.read(&mut buffer).unwrap_or_default();
        if count == 0 {
            break;
        }

        hasher.update(&buffer[..count]);
    }

    let hash = hasher.finalize();

    let mut buffer: [u8; 40] = [0; 40];

    let hash = base16ct::lower::encode_str(&hash, &mut buffer)
        .unwrap_or_default()
        .to_owned();

    let mod_file_name = path.file_name().unwrap().display().to_string();

    println!("{} -> {}", mod_file_name, hash.yellow());

    MinecraftMod::new(mod_file_name, hash)
    // (mod_file_name, hash)
}

fn check_mods_support(mc_modpack: Vec<MinecraftMod>, minecraft_version: &String, body_text: &str) {
    let body_json: Value = serde_json::from_str(body_text).unwrap_or_default();

    println!(
        "\r\n{} {}",
        "Mod(s) support check for Minecraft version".bright_cyan(),
        minecraft_version.bright_blue()
    );

    if let Some(projects) = body_json.as_object() {
        let supported_hashes: Vec<&String> = projects.keys().collect();

        let yes = "O".green();
        let no = "X".red();

        mc_modpack.iter().for_each(|mc_mod| {
            let is_available = if supported_hashes.contains(&&mc_mod.file_hash) {
                &yes
            } else {
                &no
            };
            println!(
                "{} -> {} {}",
                mc_mod.file_hash.yellow(),
                is_available,
                mc_mod.file_name
            )
        });

        let total_local_mods = mc_modpack.len();
        let total_supported_mods = projects.len();

        if total_local_mods != total_supported_mods {
            println!(
                "\r\n{} {}/{} {}",
                "Only".bright_cyan(),
                total_supported_mods.to_string().bright_blue(),
                total_local_mods.to_string().bright_blue(),
                "supported...".bright_cyan()
            )
        } else {
            println!(
                "\r\n{} {}",
                "All mods are supported for".bright_cyan(),
                minecraft_version.bright_blue()
            )
        }

        // for (hash, project) in projects {
        //     println!("hash: {}", hash);

        //     let project_id = project
        //         .get("project_id")
        //         .and_then(Value::as_str)
        //         .unwrap_or("<unknown>");
        // }
    }
}
