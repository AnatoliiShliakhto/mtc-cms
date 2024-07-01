use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Debug, Deserialize, Clone, PartialEq)]
pub struct UserDetailsModel {
    pub rank: String,
    pub name: String,
}
