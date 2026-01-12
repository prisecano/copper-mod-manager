use colored::Colorize;
use rayon::prelude::*;
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
pub mod check;

fn hash_mc_mods() -> Vec<(String, String)> {
    let mut mc_mods = Vec::new();

    for entry in WalkDir::new("mods")
        .into_iter()
        .filter_entry(|e| should_include(e))
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            mc_mods.push(entry.path().to_path_buf());
        }
    }

    hash_mods(mc_mods)
}

fn hash_mods(mc_mods: Vec<PathBuf>) -> Vec<(String, String)> {
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

fn hash_file_sha1(path: &Path) -> (String, String) {
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

    (mod_file_name, hash)
}

fn check_mods_compatible(
    mc_modpack: Vec<(String, String)>,
    minecraft_version: &String,
    body_text: String,
) {
    let body_json: Value = serde_json::from_str(body_text.as_str()).unwrap_or_default();

    println!(
        "\r\n{} {}",
        "Availability check for Minecraft version".bright_cyan(),
        minecraft_version.bright_blue()
    );

    if let Some(projects) = body_json.as_object() {
        let compatible_hashes: Vec<&String> = projects.keys().collect();

        mc_modpack.iter().for_each(|mc_mod| {
            let is_available = if compatible_hashes.contains(&&mc_mod.1) {
                "YES".green()
            } else {
                "NO".red()
            };
            println!("{} = {} ? {}", mc_mod.1.yellow(), mc_mod.0, is_available)
        });

        // for (hash, project) in projects {
        //     println!("hash: {}", hash);

        //     let project_id = project
        //         .get("project_id")
        //         .and_then(Value::as_str)
        //         .unwrap_or("<unknown>");
        // }
    }
}
