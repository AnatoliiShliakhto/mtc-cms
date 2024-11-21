use super::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthState {
    pub id: Cow<'static, str>,
    pub login: Cow<'static, str>,
    pub roles: HashSet<Cow<'static, str>>,
    pub permissions: HashSet<Cow<'static, str>>,
    pub group: Cow<'static, str>,
}

impl AuthState {
    pub fn is_authenticated(&self) -> bool {
        self.id.ne(ROLE_ANONYMOUS)
    }

    pub fn is_admin(&self) -> bool {
        self.roles.contains(ROLE_ADMINISTRATOR)
    }

    pub fn is_writer(&self) -> bool {
        self.roles.contains(ROLE_WRITER)
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(role)
    }

    pub fn has_group(&self, group: &str) -> bool {
        self.group.eq(group)
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(permission)
    }
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            id: ROLE_ANONYMOUS.into(),
            login: ROLE_ANONYMOUS.into(),
            roles: HashSet::from([ROLE_ANONYMOUS.into()]),
            permissions: HashSet::from([PERMISSION_PUBLIC_READ.into()]),
            group: "".into(),
        }
    }
}

impl From<Value> for AuthState {
    fn from(value: Value) -> Self {
        Self {
            id: value
                .key_str("id")
                .unwrap_or_default(),
            login: value
                .key_str("login")
                .unwrap_or_default(),
            roles: value
                .key_obj::<HashSet<Cow<'static, str>>>("roles")
                .unwrap_or_default(),
            group: value
                .key_str("group")
                .unwrap_or_default(),
            permissions: value
                .key_obj::<HashSet<Cow<'static, str>>>("permissions")
                .unwrap_or_default(),
        }
    }
}
