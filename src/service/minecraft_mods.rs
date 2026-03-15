use colored::Colorize;
use futures::future::join_all;
use serde_json::Value;

use crate::{
    domain::minecraft_mod::{MinecraftMod, MinecraftModVersionDiff, rules},
    infrastructure::{file_system, modrinth_api, web_api},
    presentation::view,
};

pub(crate) async fn add(mc_version: &str, id_or_slug: &str) {
    view::add::opening_message(id_or_slug);

    let mc_mod = modrinth_api::adapter::lists_projects_versions_to_mc_mod_download_url(
        mc_version, id_or_slug,
    )
    .await;

    download_and_save_mc_mod(&mc_mod.file_name, &mc_mod)
        .await
        .unwrap_or_default();
}

pub(crate) async fn latest(mc_version: &str) {
    let mut mc_mods = file_system::adapter::get_mc_mods_with_hash();

    let body =
        modrinth_api::adapter::get_latest_version_of_multiple_project(mc_version, &mut mc_mods)
            .await;

    let mc_mods_version_diff = get_latest_mods_version_diff(&body, mc_version, &mut mc_mods);

    let amount_latest_mods = mc_mods_version_diff.len();

    match view::latest::update_mods_ui(amount_latest_mods) {
        Some('a') => update_all(&mc_mods_version_diff).await,
        Some('s') => update_select(&mc_mods_version_diff).await,
        Some('n') => return,
        _ => println!("Invalid input"),
    }
}

pub(crate) fn list() {
    view::list::opening_message();

    let mc_mods = file_system::get_all_mc_mod_file_paths();

    mc_mods
        .iter()
        .for_each(|mc_mod| println!("{}", mc_mod.file_name.on_green()));
}

pub(crate) fn rm(mc_mod_file_name: &str) {
    view::rm::opening_message(mc_mod_file_name);

    match file_system::remove_mc_mod_by_file_name(mc_mod_file_name) {
        Ok(_) => {
            list();
            println!("{}", mc_mod_file_name.red())
        }
        Err(err) => println!("\r\n{err}"),
    };
}

pub(crate) async fn support(mc_version: &str) {
    view::support::opening_message(mc_version);

    let mut mc_mods = file_system::adapter::get_mc_mods_with_hash();

    let body =
        modrinth_api::adapter::get_latest_version_of_multiple_project(mc_version, &mut mc_mods)
            .await;

    let is_supported = rules::mc_mods_has_a_version_that_is_supported(&mc_mods, mc_version, &body);

    if !is_supported {
        return;
    };

    let unsupported_mc_mods = get_unsupported_mc_mods(&body, &mut mc_mods);

    if unsupported_mc_mods.is_empty() {
        view::support::mc_mods_already_supported_message(mc_version);

        return;
    }

    match view::support::update_mods_choice() {
        Some('y') => update_all(&unsupported_mc_mods).await,
        Some('n') => return,
        _ => println!("Invalid input"),
    }
}

async fn update_all(mc_mods_version_diff: &Vec<MinecraftModVersionDiff>) {
    let futures = mc_mods_version_diff.iter().map(|mc_mod_version_diff| {
        download_and_save_mc_mod(
            &mc_mod_version_diff.file_name,
            &mc_mod_version_diff.minecraft_mod_new_version,
        )
    });

    join_all(futures)
        .await
        .iter()
        .for_each(|result| match result {
            Ok(old_file_name) => {
                file_system::remove_mc_mod_by_file_name(&old_file_name)
                    .unwrap_or_else(|err| println!("{}", err));
            }
            Err(_) => {
                println!("\r\nDownload failed, Modrinth is probably down. Try again when's up.")
            }
        });
}

async fn update_select(mc_mods_version_diff: &Vec<MinecraftModVersionDiff>) {
    for mc_mod_version_diff in mc_mods_version_diff {
        view::update::select_opening_message(mc_mod_version_diff);

        loop {
            match view::update::select_choice() {
                Some('y') => {
                    if let Ok(_) = web_api::download_mc_mod_by_url(
                        &mc_mod_version_diff.minecraft_mod_new_version.download_url,
                    )
                    .await
                    {
                        file_system::remove_mc_mod_by_file_name(&mc_mod_version_diff.file_name)
                            .unwrap_or_else(|err| println!("{}", err));
                        break;
                    } else {
                        println!(
                            "\r\nDownload failed, Modrinth is probably down. Try again when's up."
                        );
                        continue;
                    }
                }
                Some('n') => break,
                _ => {
                    println!("Invalid input");
                    continue;
                }
            }
        }
    }
}

async fn download_and_save_mc_mod(
    file_name: &str,
    mc_mod: &MinecraftMod,
) -> Result<String, Box<dyn std::error::Error>> {
    println!(
        "\r\n{} {}...",
        "Downloading mod".bright_cyan(),
        mc_mod.file_name
    );

    let response = web_api::download_mc_mod_by_url(&mc_mod.download_url).await?;

    file_system::copy_downloaded_mc_mod_to_file_by_file_name(&mc_mod.file_name, response).await?;

    println!("{}", "Download successfull!".bright_green());

    Ok(file_name.to_owned())
}

fn get_latest_mods_version_diff(
    body: &Value,
    mc_version: &str,
    current_mc_mods: &mut Vec<MinecraftMod>,
) -> Vec<MinecraftModVersionDiff> {
    println!(
        "\r\n{} {}",
        "Mod(s) latest version check for Minecraft version".bright_cyan(),
        mc_version.bright_blue()
    );

    let Some(projects) = body.as_object() else {
        return vec![];
    };

    let new = " NEW! ".on_red();

    let mut mc_mods_version_diff: Vec<MinecraftModVersionDiff> = vec![];

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

        let current_mc_mod = current_mc_mods
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

        mc_mods_version_diff.push(MinecraftModVersionDiff {
            file_name: old_file_name,
            minecraft_mod_new_version: current_mc_mod.clone(),
        });
    });

    mc_mods_version_diff
}

fn get_unsupported_mc_mods(
    body: &Value,
    current_mc_mods: &mut Vec<MinecraftMod>,
) -> Vec<MinecraftModVersionDiff> {
    let Some(projects) = body.as_object() else {
        panic!();
    };
    let mut unsupported_mc_mods: Vec<MinecraftModVersionDiff> = vec![];

    projects.iter().for_each(|(current_hash, project)| {
        let file = project
            .get("files")
            .and_then(Value::as_array)
            .unwrap()
            .first()
            .unwrap_or_default();

        let current_mc_mod = current_mc_mods
            .iter_mut()
            .find(|mc_mod| &mc_mod.file_hash == current_hash)
            .unwrap();

        let old_file_name = current_mc_mod.file_name.to_owned();
        let new_file_name = file.get("filename").and_then(Value::as_str).unwrap();

        if rules::mc_mod_is_already_supported(&old_file_name, new_file_name) {
            return;
        }

        current_mc_mod.file_name = new_file_name.to_owned();
        current_mc_mod.download_url = file.get("url").and_then(Value::as_str).unwrap().to_owned();

        unsupported_mc_mods.push(MinecraftModVersionDiff {
            file_name: old_file_name,
            minecraft_mod_new_version: current_mc_mod.clone(),
        });
    });

    unsupported_mc_mods
}
