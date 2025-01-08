use super::*;

/// # Find public assets handler
///
/// This handler returns a list of assets in the specified path for the public storage.
///
/// # Permissions
///
/// The user must have the [`PERMISSION_PUBLIC_STORAGE_READ`] permission to access this handler.
///
/// # Request
///
/// The handler expects a `GET` request with a path parameter.
///
/// # Response
///
/// The handler returns a JSON response with a [`Vec`] of [`FileAsset`] objects.
///
/// # Errors
///
/// The handler returns a `400` error if the user does not have the required permission.
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

/// # Find private assets handler
///
/// This handler returns a list of assets in the specified path for the private storage.
///
/// # Permissions
///
/// The user must have the [`PERMISSION_PRIVATE_STORAGE_READ`] permission to access this handler.
///
/// # Request
///
/// The handler expects a `GET` request with a path parameter.
///
/// # Response
///
/// The handler returns a JSON response with a [`Vec`] of [`FileAsset`] objects.
///
/// # Errors
///
/// The handler returns a `400` error if the user does not have the required permission.
pub async fn find_private_assets_handler(
    Path(path): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_PRIVATE_STORAGE_READ).await?;

    state
        .repository
        .find_assets(&state.repository.get_private_dir_path(&path))
        .await?
        .to_response()
}

/// # Public Upload Handler
///
/// This handler processes file uploads to the public storage directory specified by the path.
///
/// # Permissions
///
/// The user must have the [`PERMISSION_PUBLIC_STORAGE_WRITE`] permission to upload files using this handler.
///
/// # Request
///
/// The handler expects a `POST` request with a multipart form containing the file to be uploaded.
///
/// # Response
///
/// Returns a status code `200 OK` upon successful upload.
///
/// # Errors
///
/// The handler returns a `400` error if the user does not have the required permission or if the file upload fails.
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

/// # Private Upload Handler
///
/// This handler processes file uploads to the private storage directory specified by the path.
///
/// # Permissions
///
/// The user must have the [`PERMISSION_PRIVATE_STORAGE_WRITE`] permission to upload files using this handler.
///
/// # Request
///
/// The handler expects a `POST` request with a multipart form containing the file to be uploaded.
///
/// # Response
///
/// Returns a status code `200 OK` upon successful upload.
///
/// # Errors
///
/// The handler returns a `400` error if the user does not have the required permission or if the file upload fails.
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

/// # Delete Public Asset Handler
///
/// This handler processes requests to delete a file from the public storage directory specified by the path.
///
/// # Permissions
///
/// The user must have the [`PERMISSION_PUBLIC_STORAGE_DELETE`] permission to delete files using this handler.
///
/// # Request
///
/// The handler expects a `DELETE` request with a path parameter containing the path and file name of the file to be deleted.
///
/// # Response
///
/// Returns a status code `200 OK` upon successful deletion.
///
/// # Errors
///
/// The handler returns a `400` error if the user does not have the required permission or if the file deletion fails.
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

/// # Delete Private Asset Handler
///
/// This handler processes requests to delete a file from the private storage directory specified by the path.
///
/// # Permissions
///
/// The user must have the [`PERMISSION_PRIVATE_STORAGE_DELETE`] permission to delete files using this handler.
///
/// # Request
///
/// The handler expects a `DELETE` request with a path parameter containing the path and file name of the file to be deleted.
///
/// # Response
///
/// Returns a status code `200 OK` upon successful deletion.
///
/// # Errors
///
/// The handler returns a `400` error if the user does not have the required permission or if the file deletion fails.
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
