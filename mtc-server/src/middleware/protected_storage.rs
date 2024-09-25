use super::*;

pub async fn middleware_protected_storage_handler(
    session: Session,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_PRIVATE_STORAGE_READ).await?;

    Ok(next.run(req).await)
}
