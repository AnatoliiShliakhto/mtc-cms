use super::*;

/// Opens a webpage in the default browser.
///
/// # Arguments
///
/// * `url`: The URL of the webpage to open.
///
/// # Errors
///
/// If the browser cannot be opened, an error is returned.
#[command(async, rename_all = "snake_case")]
pub async fn open_in_browser(
    url: String,
) -> Result<(), Error> {
    Ok(webbrowser::open(&url)?)
}
