use crate::prelude::*;

#[cfg(feature = "starter")]
mod starter;
#[cfg(feature = "starter")]
pub mod prelude {
    pub use super::{
        starter::prelude::*,
    };
}

#[cfg(feature = "utc242")]
mod utc242;
#[cfg(feature = "utc242")]
pub mod prelude {
    pub use super::{
        utc242::prelude::*,
    };
}

