use super::*;

#[handler(session, permission = "private_storage::read")]
pub async fn middleware_protected_storage_handler(
    req: Request,
    next: Next,
){
    Ok(next.run(req).await)
}

