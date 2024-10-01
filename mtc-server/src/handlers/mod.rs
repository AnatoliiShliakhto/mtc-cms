use crate::prelude::*;

mod sync;
mod auth;
mod permissions;
mod groups;
mod roles;
mod users;
mod schemas;

pub mod prelude {
    pub use super::{
        auth::*,
        sync::*,
        permissions::*,
        groups::*,
        roles::*,
        users::*,
        schemas::*,
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