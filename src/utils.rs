use crate::{
    entity::{MinecraftMod, MinecraftModVersionUpdate},
    modrinth_api,
    service::MinecraftModsService,
};
use colored::Colorize;
use futures::future::join_all;
use rayon::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::{error::Error, fs, io::copy};
use walkdir::{DirEntry, WalkDir};

pub(crate) async fn get_latest_version_of_multiple_project(
    mc_version: &str,
    mc_mods_service: &mut MinecraftModsService,
) -> Value {
    let game_versions = &vec![mc_version];
    let loaders = &vec!["fabric"];

    println!(
        "\r\n{}",
        "Obtaining hashes for Modrinth lookup:".bright_cyan()
    );
    get_mod_file_paths(mc_mods_service);
    parallise_hashing_mc_mods(mc_mods_service);

    let mods_hashes: &Vec<&str> = &mc_mods_service
        .mc_mods
        .iter()
        .map(|mc_mod| mc_mod.file_hash.as_str())
        .collect();

    println!("\r\n{}", "Searching mods on Modrinth...".bright_cyan());

    let body =
        modrinth_api::latest_version_of_multiple_project(mods_hashes, loaders, game_versions)
            .await
            .unwrap_or_default();

    println!("\r\n{}", "Done!".bright_green());
    body
}

pub(crate) async fn add_mc_mod(mc_mod: &MinecraftMod) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}...",
        "Downloading mod".bright_cyan(),
        mc_mod.file_name
    );

    let response = download_mc_mod_by_url(&mc_mod.download_url).await?;
    copy_downloaded_mc_mod_to_file_by_file_name(&mc_mod.file_name, response).await?;

    Ok(())
}

pub(crate) async fn update_mc_mod_to_new_version(
    mc_mod: &MinecraftModVersionUpdate,
) -> Result<String, Box<dyn std::error::Error>> {
    add_mc_mod(&mc_mod.minecraft_mod_new_version).await?;

    println!(
        "{} -> {}",
        "update complete!".bright_green(),
        mc_mod.minecraft_mod_new_version.file_name,
    );

    Ok(mc_mod.file_name.to_owned())
}

async fn copy_downloaded_mc_mod_to_file_by_file_name(
    file_name: &str,
    mut response: reqwest::Response,
) -> Result<(), Box<dyn Error + 'static>> {
    let mut dest = File::create(format!("mods/{file_name}")).unwrap();
    while let Some(chunk) = response.chunk().await? {
        copy(&mut chunk.as_ref(), &mut dest).unwrap();
    }

    Ok(())
}

async fn download_mc_mod_by_url(
    file_download_url: &str,
) -> Result<reqwest::Response, Box<dyn Error + 'static>> {
    let response = reqwest::get(file_download_url).await?;
    Ok(response)
}

pub(crate) fn get_mod_file_paths(mc_mods_service: &mut MinecraftModsService) {
    for entry in WalkDir::new("mods")
        .into_iter()
        .filter_entry(|e| should_include(e))
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            mc_mods_service
                .mc_mods
                .push(MinecraftMod::new_mc_mod_by_path(entry.into_path()));
        }
    }
}

pub(crate) fn parallise_hashing_mc_mods(mc_mods_service: &mut MinecraftModsService) {
    mc_mods_service
        .mc_mods
        .par_iter_mut()
        .for_each(|mc_mod| mc_mod.hash_file_sha1());
}

pub(crate) fn should_include(entry: &DirEntry) -> bool {
    if entry.file_type().is_dir() {
        return true;
    }

    match entry.path().extension().and_then(|e| e.to_str()) {
        Some("jar") => true,
        _ => false,
    }
}

pub(crate) fn check_support_mc_mods(
    mc_mods_service: &MinecraftModsService,
    mc_version: &str,
    body: &Value,
) -> bool {
    println!(
        "\r\n{} {}",
        "Mod(s) support check for".bright_cyan(),
        mc_version.bright_blue()
    );

    let Some(projects) = body.as_object() else {
        return false;
    };
    let supported_hashes: Vec<&String> = projects.keys().collect();

    let yes = "O".green();
    let no = "X".red();

    mc_mods_service.mc_mods.iter().for_each(|mc_mod| {
        let is_available = match supported_hashes.contains(&&mc_mod.file_hash) {
            true => &yes,
            false => &no,
        };
        println!(
            "{} -> {} {}",
            mc_mod.file_hash.yellow(),
            is_available,
            mc_mod.file_name
        )
    });

    let total_local_mods = mc_mods_service.mc_mods.len();
    let total_supported_mods = projects.len();

    if total_local_mods != total_supported_mods {
        println!(
            "\r\n{} {}/{} {}",
            "Only".bright_cyan(),
            total_supported_mods.to_string().bright_blue(),
            total_local_mods.to_string().bright_blue(),
            "supported...".bright_cyan()
        );

        return false;
    }

    println!(
        "\r\n{} {}",
        "All mods has versions that support".bright_cyan(),
        mc_version.bright_blue()
    );

    true
}

pub(crate) async fn check_latest_mc_mods(
    body: &Value,
    mc_version: &str,
    current_mc_mods_service: &mut MinecraftModsService,
) {
    println!(
        "\r\n{} {}",
        "Mod(s) latest version check for Minecraft version".bright_cyan(),
        mc_version.bright_blue()
    );

    let Some(projects) = body.as_object() else {
        return;
    };

    let new = " NEW! ".on_red();

    let mut mc_mods_version_update: Vec<MinecraftModVersionUpdate> = vec![];

    projects.iter().for_each(|(current_hash, project)| {
        let file = project
            .get("files")
            .and_then(Value::as_array)
            .unwrap()
            .first()
            .unwrap_or_default();
        let file_hashes = file.get("hashes").and_then(Value::as_object).unwrap();
        let latest_hash = file_hashes.get("sha1").and_then(Value::as_str).unwrap();

        if current_hash == latest_hash {
            return;
        }

        let current_mc_mod = current_mc_mods_service
            .mc_mods
            .iter_mut()
            .find(|mc_mod| &mc_mod.file_hash == current_hash)
            .unwrap();

        let old_file_name = current_mc_mod.file_name.to_owned();
        let new_file_name = file.get("filename").and_then(Value::as_str).unwrap();
        println!(
            "\r\n{} -> {} {new}",
            old_file_name.blue(),
            new_file_name.bright_blue()
        );

        let changelog = project.get("changelog").and_then(Value::as_str).unwrap();
        println!("{}\r\n{}", "Changelog:".bright_cyan(), changelog.on_black());

        let download_url = file.get("url").and_then(Value::as_str).unwrap();

        current_mc_mod.file_name = new_file_name.to_owned();
        current_mc_mod.file_hash = current_hash.to_owned();
        current_mc_mod.changelog = changelog.to_owned();
        current_mc_mod.download_url = download_url.to_owned();

        mc_mods_version_update.push(MinecraftModVersionUpdate {
            file_name: old_file_name,
            minecraft_mod_new_version: current_mc_mod.clone(),
        });
    });

    let n = mc_mods_version_update.len();
    if n == 0 {
        println!("\r\n{}", "Mods are up-to-date!".bright_green())
    } else {
        println!(
            "\r\n{}",
            format!("There are {n} mod(s) with newer version!").bright_yellow()
        );
        println!("\r\nUpdate [a]ll, [s]elect, [n]one?");

        let mut update_choice = String::new();
        std::io::stdin()
            .read_line(&mut update_choice)
            .unwrap_or_default();

        match update_choice.trim().chars().next() {
            Some('a') => update_all(&mc_mods_version_update).await,
            Some('s') => update_selective(&mc_mods_version_update).await,
            Some('n') => return,
            _ => println!("Invalid input"),
        }
    }
}

async fn update_all(new_mc_mods: &[MinecraftModVersionUpdate]) {
    let futures = new_mc_mods
        .iter()
        .map(|new_mc_mod| update_mc_mod_to_new_version(&new_mc_mod));

    join_all(futures)
        .await
        .iter()
        .for_each(|result| match result {
            Ok(old_file_name) => {
                remove_mc_mod_by_file_name(&old_file_name)
                    .unwrap_or_else(|err| println!("{}", err));
            }
            Err(_) => {
                println!("\r\nDownload failed, Modrinth is probably down. Try again when's up.")
            }
        });
}

async fn update_selective(new_mc_mods: &[MinecraftModVersionUpdate]) {
    for new_mc_mod in new_mc_mods {
        println!(
            "\r\n{} -> {}",
            new_mc_mod.file_name.bright_blue(),
            new_mc_mod.minecraft_mod_new_version.file_name.bright_blue()
        );
        println!(
            "{}\r\n{}",
            "Changelog:".bright_cyan(),
            new_mc_mod.minecraft_mod_new_version.changelog.on_black()
        );

        loop {
            println!("\r\n{} [y]es, [n]o:", "Update this mod?".bright_cyan());

            let mut mc_mod_choice = String::new();
            std::io::stdin()
                .read_line(&mut mc_mod_choice)
                .unwrap_or_default();

            match mc_mod_choice.trim().chars().next() {
                Some('y') => match add_mc_mod(&new_mc_mod.minecraft_mod_new_version).await {
                    Ok(_) => {
                        remove_mc_mod_by_file_name(&new_mc_mod.file_name)
                            .unwrap_or_else(|err| println!("{}", err));
                        break;
                    }
                    Err(_) => {
                        println!(
                            "\r\nDownload failed, Modrinth is probably down. Try again when's up."
                        );
                        continue;
                    }
                },
                Some('n') => break,
                _ => {
                    println!("Invalid input");
                    continue;
                }
            }
        }
    }
}

pub(crate) fn remove_mc_mod_by_file_name(
    mc_mod_file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_file("mods/".to_string() + mc_mod_file_name)?;
    Ok(())
}

pub(crate) async fn update_mods_to_support_a_mc_version_ui(
    body: &Value,
    mc_version: &str,
    current_mc_mods: &mut MinecraftModsService,
) {
    loop {
        println!("\r\nUpdate mod(s)? [y]es, [n]o");

        let mut update_choice = String::new();
        std::io::stdin()
            .read_line(&mut update_choice)
            .unwrap_or_default();

        match update_choice.trim().chars().next() {
            Some('y') => {
                println!(
                    "\r\n{} {}",
                    "Update Mod(s) to support".bright_cyan(),
                    mc_version.bright_blue()
                );
                update_mods_to_support_a_mc_version(body, current_mc_mods).await;

                break;
            }
            Some('n') => break,
            _ => println!("Invalid input"),
        }
    }
}

async fn update_mods_to_support_a_mc_version(
    body: &Value,
    current_mc_mods_service: &mut MinecraftModsService,
) {
    let Some(projects) = body.as_object() else {
        return;
    };

    let mut mc_mods_version_update: Vec<MinecraftModVersionUpdate> = vec![];

    projects.iter().for_each(|(current_hash, project)| {
        let file = project
            .get("files")
            .and_then(Value::as_array)
            .unwrap()
            .first()
            .unwrap_or_default();

        let current_mc_mod = current_mc_mods_service
            .mc_mods
            .iter_mut()
            .find(|mc_mod| &mc_mod.file_hash == current_hash)
            .unwrap();

        let old_file_name = current_mc_mod.file_name.to_owned();
        let new_file_name = file.get("filename").and_then(Value::as_str).unwrap();

        if old_file_name == new_file_name {
            return;
        }

        current_mc_mod.file_name = new_file_name.to_owned();
        current_mc_mod.download_url = file.get("url").and_then(Value::as_str).unwrap().to_owned();

        mc_mods_version_update.push(MinecraftModVersionUpdate {
            file_name: old_file_name,
            minecraft_mod_new_version: current_mc_mod.clone(),
        });
    });

    if mc_mods_version_update.is_empty() {
        println!(
            "\r\n{}",
            "Mod(s) already has the correct version!".bright_green()
        );
        return;
    }

    update_all(&mc_mods_version_update).await;
}
