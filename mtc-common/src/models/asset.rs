use super::*;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub name: Cow<'static, str>,
    pub size: usize,
}
