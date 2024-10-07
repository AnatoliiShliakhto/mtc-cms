use crate::prelude::*;

#[cfg(starter)]
mod starter;
#[cfg(starter)]
pub mod prelude {
    pub use super::{
        starter::prelude::*,
    };
}

#[cfg(utc242)]
mod utc242;
#[cfg(utc242)]
pub mod prelude {
    pub use super::{
        utc242::prelude::*,
    };
}

