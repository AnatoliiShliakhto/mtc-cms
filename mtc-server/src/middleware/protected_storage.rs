use super::*;

/// Checks if the user has the [`PERMISSION_PRIVATE_STORAGE_READ`] permission before allowing them to access the next handler.
///
/// # Arguments
///
/// * `session` - The current user session.
/// * `req` - The request to be processed.
/// * `next` - The next handler to be called.
///
/// # Returns
///
/// * `Result<impl IntoResponse>` - The result of the next handler, or an error if the user does not have the required permission.
pub async fn middleware_protected_storage_handler(
    session: Session,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_PRIVATE_STORAGE_READ).await?;

    Ok(next.run(req).await)
}

