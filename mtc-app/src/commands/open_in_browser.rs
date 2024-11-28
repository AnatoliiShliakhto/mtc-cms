use super::*;

#[command(async, rename_all = "snake_case")]
pub async fn open_in_browser(
    url: String,
) -> Result<(), Error> {
    Ok(webbrowser::open(&url)?)
}
