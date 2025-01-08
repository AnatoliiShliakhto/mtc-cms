use super::*;

/// User details structure
#[derive(Serialize, Default, Debug, Deserialize, Clone, PartialEq)]
pub struct UserDetails {
    pub id: Cow<'static, str>,
    pub login: Cow<'static, str>,
    pub rank: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub group: Cow<'static, str>,
    pub state: UserState,
    pub password: Cow<'static, str>,
    pub last_access: Option<Datetime>,
    pub access_count: i32,
}

/// DTO for [`UserDetails`] structure
#[derive(Serialize, Default, Debug, Deserialize, Clone, PartialEq)]
pub struct UserDetailsDto {
    pub id: Cow<'static, str>,
    pub login: Cow<'static, str>,
    pub group: Cow<'static, str>,
    pub password: Cow<'static, str>,
    pub blocked: bool,
    pub last_access: Option<Datetime>,
    pub access_count: i32,
}

/// User state enum
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserState {
    #[default]
    Unknown,
    Active,
    Inactive,
}

/// Person details DTO
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersonDto {
    pub login: Cow<'static, str>,
    pub rank: Cow<'static, str>,
    pub name: Cow<'static, str>,
}