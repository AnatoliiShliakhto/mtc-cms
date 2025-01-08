use super::*;

/// Downloads a file from a specified URL and saves it to a given path.
///
/// # Arguments
///
/// * `app` - The application handle used to access the filesystem.
/// * `state` - The application state containing the HTTP client.
/// * `url` - The URL of the file to be downloaded.
/// * `path` - The destination path where the downloaded file will be saved.
///
/// # Returns
///
/// * `Result<(), `[`Error`]`>` - Returns an empty result on success, or an error if the download or file write fails.
///
/// # Errors
///
/// Returns an error if the HTTP request fails or if the response status code is not `200 OK`.
/// Additionally, returns an error if writing to the file fails.
#[command(async)]
pub async fn download(
    app: AppHandle,
    state: State<'_, AppState>,
    url: String,
    path: String
) -> Result<(), Error> {
    let response = state.client.lock().await.get(&url).send().await?;

    if response.status() != StatusCode::OK {
        Err(Error::Generic(format!("Response status code: {}", response.status())))?
    };

    let mut options = tauri_plugin_fs::OpenOptions::new();
    options.create(true).write(true).truncate(true);

    let mut file = app.fs().open(PathBuf::from(path), options)?;
    file.write(response.bytes().await?.as_ref())?;

    Ok(())
}
