use sha1::{Digest, Sha1};
// use std::collections::HashMap;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn hash_mc_mods() -> Vec<String> {
    let mut mc_mods: Vec<_> = Vec::new();
    // let mut modpack = HashMap::new();

    for entry in WalkDir::new("mods")
        .into_iter()
        .filter_entry(|e| should_include(e))
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            mc_mods.push(entry.path().to_path_buf());
            // let hash = hash_file_sha1(path).unwrap();
            // println!("{} -> {}", path.file_name().unwrap().display(), hash);
            // mods_hashes.push(hash);
        }
    }
    println!();

    hash_mods(mc_mods)
}

fn hash_mods(mc_mods: Vec<std::path::PathBuf>) -> Vec<String> {
    mc_mods
        .par_iter()
        .map(|path| hash_file_sha1(path.as_path()))
        .collect()
}

pub fn should_include(entry: &DirEntry) -> bool {
    if entry.file_type().is_dir() {
        return true;
    }

    // file extensions filter
    match entry.path().extension().and_then(|e| e.to_str()) {
        Some("jar") => true,
        _ => false,
    }
}

pub fn hash_file_sha1(path: &Path) -> String {
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

    println!("{} -> {}", path.file_name().unwrap().display(), hash);

    hash
}
