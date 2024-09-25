use crate::prelude::*;

mod header;
mod main_menu;
mod footer;
mod side_menu;

pub mod prelude {
    pub use super::{
        header::*,
        footer::*,
        main_menu::*,
        side_menu::*,
    };
}

