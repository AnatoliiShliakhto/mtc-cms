use crate::prelude::*;

mod sync;
mod auth;
mod permissions;
mod groups;
mod roles;

pub mod prelude {
    pub use super::{
        auth::*,
        sync::*,
        permissions::*,
        groups::*,
        roles::*,
    };
}

pub trait HandlerTrait {
    fn to_response(self) -> Result<impl IntoResponse>;
}

impl<T: Serialize + Sized> HandlerTrait for T
{
    fn to_response(self) -> Result<impl IntoResponse> {
        Ok((StatusCode::OK, Json(self)).into_response())
    }
}