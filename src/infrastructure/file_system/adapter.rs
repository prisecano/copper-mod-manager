use colored::Colorize;
use rayon::prelude::*;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{BufReader, Read};

use crate::domain::minecraft_mod::MinecraftMod;

pub(crate) fn get_mc_mods_with_hash() -> Vec<MinecraftMod> {
    let mut mc_mods = super::get_all_mc_mod_file_paths();

    println!(
        "\r\n{}",
        "Obtaining hashes for Modrinth lookup:".bright_cyan()
    );

    mc_mods
        .par_iter_mut()
        .for_each(|mc_mod| hash_file_sha1(mc_mod));

    mc_mods
}

fn hash_file_sha1(mc_mod: &mut MinecraftMod) {
    let file = File::open(mc_mod.file_path.as_path()).unwrap();
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

    mc_mod.file_hash =
        String::from(base16ct::lower::encode_str(&hash, &mut buffer).unwrap_or_default());

    println!("{} -> {}", mc_mod.file_name, mc_mod.file_hash.yellow());
}
