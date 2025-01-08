use super::*;

/// Check that the request contains a session header.
///
/// If the request does not contain a session header, return a `400 Bad Request` error.
/// Otherwise, call the next handler with the request.
///
pub async fn middleware_headers_check_handler(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    if req.headers().get("session").is_none() { Err(GenericError::BadRequest)? }

    Ok(next.run(req).await)
}