use super::*;

#[handler(session, permission = "storage::read")]
pub async fn find_public_assets_handler(
    Path(path): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
) {
    state
        .repository
        .find_assets(&state.repository.get_public_dir_path(&path))
        .await
        .map(Json)
}

#[handler(session, permission = "private_storage::read")]
pub async fn find_private_assets_handler(
    Path(path): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
) {
    state
        .repository
        .find_assets(&state.repository.get_private_dir_path(&path))
        .await
        .map(Json)
}

#[handler(session, permission = "storage::write", result)]
pub async fn public_upload_handler(
    Path(path): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    mut multipart: Multipart,
) {
    let path = state.repository.get_public_dir_path(&path);

    state.repository.is_dir_exists_or_create(&path).await?;

    while let Some(field) = multipart.next_field().await? {
        if Some("file") == field.name() && field.file_name().is_some() {
            state.repository.upload_asset(&path, field).await?
        }
    }
}

#[handler(session, permission = "private_storage::write", result)]
pub async fn private_upload_handler(
    Path(path): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    mut multipart: Multipart,
) {
    let path = state.repository.get_private_dir_path(&path);

    state.repository.is_dir_exists_or_create(&path).await?;

    while let Some(field) = multipart.next_field().await? {
        if Some("file") == field.name() && field.file_name().is_some() {
            state.repository.upload_asset(&path, field).await?
        }
    }
}

#[handler(session, permission = "storage::delete")]
pub async fn delete_public_asset_handler(
    Path((path, file)): Path<(Cow<'static, str>, Cow<'static, str>)>,
    state: State<Arc<AppState>>,
) {
    state
        .repository
        .delete_file(&state.repository.get_public_asset_path(&path, &file))
        .await
}

#[handler(session, permission = "private_storage::delete")]
pub async fn delete_private_asset_handler(
    Path((path, file)): Path<(Cow<'static, str>, Cow<'static, str>)>,
    state: State<Arc<AppState>>,
) {
    state
        .repository
        .delete_file(&state.repository.get_private_asset_path(&path, &file))
        .await
}
