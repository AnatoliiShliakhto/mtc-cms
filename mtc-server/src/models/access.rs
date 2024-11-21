use super::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Access {
    pub level: i32,
    pub full: bool,
}

impl Access {
    pub fn administrator() -> Self {
        Self {
            level: -1,
            full: true,
        }
    }
}

impl Default for Access {
    fn default() -> Self {
        Self {
            level: 999,
            full: false,
        }
    }
}

impl From<Value> for Access {
    fn from(value: Value) -> Self {
        Self {
            level: value.key_i64("access_level").unwrap_or(999) as i32,
            full: value.key_bool("full_access").unwrap_or_default()
        }
    }
}