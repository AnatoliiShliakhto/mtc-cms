use super::*;

pub async fn find_public_assets_handler(
    Path(path): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_PUBLIC_STORAGE_READ).await?;

    let assets = state
        .repository
        .find_assets(&state.repository.get_public_dir_path(&path))
        .await?;

    assets.to_response()
}

pub async fn find_private_assets_handler(
    Path(path): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_PRIVATE_STORAGE_READ).await?;

    let assets = state
        .repository
        .find_assets(&state.repository.get_private_dir_path(&path))
        .await?;

    assets.to_response()
}

pub async fn public_upload_handler(
    Path(path): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_PUBLIC_STORAGE_WRITE).await?;

    let path = state.repository.get_public_dir_path(&path);

    state.repository.is_dir_exists_or_create(&path).await?;

    while let Some(field) = multipart.next_field().await? {
        if Some("file") == field.name() && field.file_name().is_some() {
            state.repository.upload_asset(&path, field).await?
        }
    }

    Ok(StatusCode::OK)
}

pub async fn private_upload_handler(
    Path(path): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_PRIVATE_STORAGE_WRITE).await?;

    let path = state.repository.get_private_dir_path(&path);

    state.repository.is_dir_exists_or_create(&path).await?;

    while let Some(field) = multipart.next_field().await? {
        if Some("file") == field.name() && field.file_name().is_some() {
            state.repository.upload_asset(&path, field).await?
        }
    }

    Ok(StatusCode::OK)
}

pub async fn delete_public_asset_handler(
    Path((path, file)): Path<(Cow<'static, str>, Cow<'static, str>)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_PUBLIC_STORAGE_DELETE).await?;

    state
        .repository
        .delete_file(&state.repository.get_public_asset_path(&path, &file))
        .await?;

    Ok(StatusCode::OK)
}

pub async fn delete_private_asset_handler(
    Path((path, file)): Path<(Cow<'static, str>, Cow<'static, str>)>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_PRIVATE_STORAGE_DELETE).await?;

    state
        .repository
        .delete_file(&state.repository.get_private_asset_path(&path, &file))
        .await?;

    Ok(StatusCode::OK)
}
