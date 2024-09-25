use super::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Payload<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Payload<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection=JsonRejection>,
    Form<T>: FromRequest<S, Rejection=FormRejection>,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> std::result::Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(payload) = Json::<T>::from_request(req, state).await?;
                //payload.validate()?;
                return Ok(Self(payload));
            }

            if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(payload) = Form::<T>::from_request(req, state).await?;
                //payload.validate()?;
                return Ok(Self(payload));
            }
        }

        Err(Self::Rejection::from(GenericError::UnsupportedMediaType))
    }
}
