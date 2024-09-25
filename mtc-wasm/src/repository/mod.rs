use crate::prelude::*;

mod local_storage;
mod session_storage;

pub mod prelude {
    pub use super::{
        local_storage::*,
        session_storage::*,
    };
}

pub struct StorageEntry<T> {
    key: Cow<'static, str>,
    value: T,
}