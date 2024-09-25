use prelude::*;

mod consts;
mod models;
mod utils;

pub mod prelude {
    pub use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
    pub use serde_json::value::Value;
    pub use serde_repr::{Serialize_repr, Deserialize_repr};
    pub use std::{borrow::Cow, collections::HashSet, str::FromStr, fmt::Display};
    pub use surrealdb::sql::{Datetime, Thing};
    pub use chrono::{DateTime, Local};
    pub use super::{
        consts::prelude::*,
        models::prelude::*,
        utils::prelude::*,
    };
}


///todo redundant
pub fn from_thing<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Cow<'static, str>, D::Error>
{
    #[derive(Deserialize, Serialize, Debug)]
    #[serde(untagged)]
    enum Helper {
        #[serde(with = "Thing")]
        Thing(Thing),
        String(String),
    }

    match Helper::deserialize(deserializer)? {
        Helper::String(value) => Ok(Cow::Owned(value)),
        Helper::Thing(value) => Ok(Cow::Owned(value.id.to_string()))
    }

    //Ok(Cow::Owned(Thing::deserialize(deserializer)?.id.to_string()))
}
