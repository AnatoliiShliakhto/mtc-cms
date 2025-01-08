use super::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Payload<T>(pub T);

impl<T, S> FromRequest<S> for Payload<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection=JsonRejection>,
    Form<T>: FromRequest<S, Rejection=FormRejection>,
{
    type Rejection = Error;

    /// Extracts the payload from the request body. Supports the following content types:
    ///
    /// * `application/json`
    /// * `application/x-www-form-urlencoded`
    ///
    /// If the content type is not supported, returns a `GenericError::UnsupportedMediaType` error.
    async fn from_request(req: Request, state: &S) -> std::result::Result<Self, Self::Rejection> {
        match req.headers().get(CONTENT_TYPE).and_then(|value| value.to_str().ok()) {
            Some(content_type) if content_type.starts_with("application/json") => {
                let Json(payload) = Json::<T>::from_request(req, state).await?;
                Ok(Self(payload))
            }
            Some(content_type) if content_type.starts_with("application/x-www-form-urlencoded") => {
                let Form(payload) = Form::<T>::from_request(req, state).await?;
                Ok(Self(payload))
            }
            _ => Err(Self::Rejection::from(GenericError::UnsupportedMediaType)),
        }
    }
}
