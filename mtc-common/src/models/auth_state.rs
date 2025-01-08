use super::*;

/// Represents the authentication state of a user.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthState {
    pub id: Cow<'static, str>,
    pub login: Cow<'static, str>,
    pub roles: HashSet<Cow<'static, str>>,
    pub permissions: HashSet<Cow<'static, str>>,
    pub group: Cow<'static, str>,
}

impl AuthState {
    /// Checks if the user is authenticated.
    ///
    /// # Returns
    ///
    /// * `true` if the user is authenticated.
    /// * `false` otherwise.
    pub fn is_authenticated(&self) -> bool {
        self.id.ne(ROLE_ANONYMOUS)
    }

    /// Checks if the user has the "administrator" role.
    ///
    /// # Returns
    ///
    /// * `true` if the user has the "administrator" role.
    /// * `false` otherwise.
    pub fn is_admin(&self) -> bool {
        self.roles.contains(ROLE_ADMINISTRATOR)
    }

    /// Checks if the user has the "writer" role.
    ///
    /// # Returns
    ///
    /// * `true` if the user has the "writer" role.
    /// * `false` otherwise.
    pub fn is_writer(&self) -> bool {
        self.roles.contains(ROLE_WRITER)
    }

    /// Checks if the user has a specific role.
    ///
    /// # Parameters
    ///
    /// - `role`: A string slice that holds the role to check.
    ///
    /// # Returns
    ///
    /// * `true` if the user has the specified role.
    /// * `false` otherwise.
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(role)
    }

    /// Checks if the user is a member of a specific group.
    ///
    /// # Parameters
    ///
    /// - `group`: A string slice that holds the group to check.
    ///
    /// # Returns
    ///
    /// * `true` if the user is a member of the specified group.
    /// * `false` otherwise.
    pub fn has_group(&self, group: &str) -> bool {
        self.group.eq(group)
    }

    /// Checks if the user has a specific permission.
    ///
    /// # Parameters
    ///
    /// - `permission`: A string slice that holds the permission to check.
    ///
    /// # Returns
    ///
    /// * `true` if the user has the specified permission.
    /// * `false` otherwise.
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
    /// Constructs an [`AuthState`] from a [`Value`] object.
    ///
    /// This function extracts the fields `id`, `login`, `roles`, `group`, and `permissions`
    /// from the provided [`serde_json::value::Value`] object and uses them to create a new [`AuthState`] instance.
    /// Default values are used if any field is missing or cannot be parsed.
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
