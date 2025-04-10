use super::*;

pub async fn middleware_headers_check_handler(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    if !req.headers().contains_key("session") { Err(GenericError::BadRequest)? }

    Ok(next.run(req).await)
}