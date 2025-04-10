use super::*;

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
