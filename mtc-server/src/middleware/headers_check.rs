use super::*;

pub async fn middleware_headers_check_handler(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    if !req.headers().contains_key("session") {
        return Err(GenericError::BadRequest.into());
    }

    Ok(next.run(req).await)
}