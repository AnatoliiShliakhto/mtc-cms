use axum::{async_trait, Form, Json};
use axum::extract::{FromRequest, Request};
use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::http::header::CONTENT_TYPE;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::api_error::ApiError;
use crate::error::generic_error::GenericError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedPayload<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedPayload<T>
    where
        T: DeserializeOwned + Validate,
        S: Send + Sync,
        Json<T>: FromRequest<S, Rejection=JsonRejection>,
        Form<T>: FromRequest<S, Rejection=FormRejection>,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(payload) = Json::<T>::from_request(req, state).await?;
                payload.validate()?;
                return Ok(Self(payload));
            }

            if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(payload) = Form::<T>::from_request(req, state).await?;
                payload.validate()?;
                return Ok(Self(payload));
            }
        }

        Err(Self::Rejection::from(GenericError::UnsupportedMediaType))
    }
}

