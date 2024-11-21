use super::*;

#[derive(Deserialize)]
pub struct QueryUuid {
    pub uuid: Cow<'static, str>,
}