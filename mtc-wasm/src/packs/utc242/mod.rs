use super::*;

mod breadcrumbs;
mod layout;
mod router;
mod menu;
mod home;

pub mod prelude {
    pub static I18N_EN_US: &str = include_str!("i18n/en-US.ftl");
    pub static I18N_UK_UA: &str = include_str!("i18n/uk-UA.ftl");

    pub use super::{
        breadcrumbs::*,
        layout::*,
        router::*,
        menu::*,
        home::*,
    };
}



