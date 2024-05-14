use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthModel {
    pub id: String,
    pub roles: Vec<String>,
    pub groups: Vec<String>,
    pub permissions: Vec<String>,
}

pub trait AuthModelTrait {
    fn is_role(&self, role: &str) -> bool;
    fn is_group(&self, group: &str) -> bool;
    fn is_permission(&self, permission: &str) -> bool;
}

impl AuthModelTrait for AuthModel {
    fn is_role(&self, role: &str) -> bool {
        self.roles.iter().any(|item| item == role)
    }

    fn is_group(&self, group: &str) -> bool {
        self.groups.iter().any(|item| item == group)
    }

    fn is_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|item| item == permission)
    }
}

#[derive(Deserialize)]
pub struct SignInModel {
    pub login: String,
    pub password: String,
}