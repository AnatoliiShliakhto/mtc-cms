#[cfg(not(debug_assertions))]
pub const API_URL: &str = "https://242.org.ua/api";
#[cfg(debug_assertions)]
pub const API_URL: &str = "https://localhost/api";

pub const API_PATH: &str = "/api";
pub const PUBLIC_STORAGE_PATH: &str = "/public"; // path to uploaded files
pub const PROTECTED_STORAGE_PATH: &str = "/protected"; // API endpoint for protected files

pub const ID_CREATE: &str = "create";
pub const SLUG_PATTERN: &str = "[\\d\\w\\-]{4,50}"; // [word, digit, -] {min, max}
pub const TITLE_PATTERN: &str = ".{4,250}"; // any {min, max}
