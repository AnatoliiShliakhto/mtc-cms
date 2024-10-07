use super::*;

mod breadcrumbs;
mod layout;
mod router;
mod settings;
mod menu;
mod home;

pub mod prelude {
    pub use super::{
        breadcrumbs::*,
        layout::*,
        router::*,
        settings::*,
        menu::*,
        home::*,
    };
}



