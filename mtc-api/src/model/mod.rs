use serde::{Deserialize, Deserializer};
use surrealdb::sql::Thing;

pub mod role_model;
pub mod permission_model;
pub mod response_model;
pub mod request_model;
pub mod user_model;

pub fn from_thing<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    let thing: Thing = Deserialize::deserialize(deserializer)?;
    Ok(thing.id.to_string())
}