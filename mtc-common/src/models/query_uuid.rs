use super::*;

/// Query uuid structure
#[derive(Deserialize)]
pub struct QueryUuid {
    pub uuid: Cow<'static, str>,
}