use std::fs::File;
use std::{error::Error, fs, io::copy};

pub(crate) async fn copy_downloaded_mc_mod_to_file_by_file_name(
    file_name: &str,
    mut response: reqwest::Response,
) -> Result<(), Box<dyn Error + 'static>> {
    let mut dest = File::create(format!("mods/{file_name}")).unwrap();
    while let Some(chunk) = response.chunk().await? {
        copy(&mut chunk.as_ref(), &mut dest).unwrap();
    }

    Ok(())
}

pub(crate) fn remove_mc_mod_by_file_name(
    mc_mod_file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_file("mods/".to_string() + mc_mod_file_name)?;
    Ok(())
}
