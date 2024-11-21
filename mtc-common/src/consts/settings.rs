pub static PUBLIC_ASSETS_PATH: &str = "/public"; // path to uploaded files
pub static PRIVATE_ASSETS_PATH: &str = "/protected"; // API endpoint for protected files

pub static ID_CREATE: &str = "create";

pub static SLUG_PATTERN: &str = "[\\d\\w\\-]{4,50}"; // [word, digit, -] {min, max}
pub static TITLE_PATTERN: &str = ".{4,250}"; // any {min, max}
pub static UUID_PATTERN: &str = "^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"; // UUID v4
