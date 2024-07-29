use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccessModel {
    pub users_level: i32,
    pub users_all: bool,
}

impl Default for AccessModel {
    fn default() -> Self {
        Self {
            users_level: 999,
            users_all: false,
        }
    }
}
