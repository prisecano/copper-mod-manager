use std::error::Error;

pub(crate) async fn download_mc_mod_by_url(
    file_download_url: &str,
) -> Result<reqwest::Response, Box<dyn Error + 'static>> {
    let response = reqwest::get(file_download_url).await?;
    Ok(response)
}
