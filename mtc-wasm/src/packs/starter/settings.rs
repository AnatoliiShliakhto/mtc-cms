#[cfg(not(debug_assertions))]
pub static API_ENDPOINT: &str = "https://your.site.domain/api";
#[cfg(debug_assertions)]
pub static API_ENDPOINT: &str = "https://localhost/api";

pub static I18N_EN_US: &str = include_str!("i18n/en-US.ftl");
pub static I18N_UK_UA: &str = include_str!("i18n/uk-UA.ftl");