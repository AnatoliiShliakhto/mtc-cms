use super::*;

#[derive(Serialize, Deserialize)]
pub struct Access {
    pub level: i32,
    pub full: bool,
}

impl Default for Access {
    fn default() -> Self {
        Self {
            level: 999,
            full: false,
        }
    }
}
