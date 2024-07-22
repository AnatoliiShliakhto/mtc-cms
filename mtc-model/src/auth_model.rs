use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthModel {
    pub id: String,
    pub roles: Vec<String>,
    pub groups: Vec<String>,
    pub permissions: Vec<String>,
}

impl Default for AuthModel {
    fn default() -> Self {
        Self {
            id: "anonymous".to_string(),
            roles: vec!["anonymous".to_string()],
            groups: vec![],
            permissions: vec!["content::read".to_string()],
        }
    }
}

pub trait AuthModelTrait {
    fn is_auth(&self) -> bool;
    fn is_admin(&self) -> bool;
    fn is_role(&self, role: &str) -> bool;
    fn is_group(&self, group: &str) -> bool;
    fn is_permission(&self, permission: &str) -> bool;
}

impl AuthModelTrait for AuthModel {
    fn is_auth(&self) -> bool {
        !self.id.eq("anonymous")
    }

    fn is_admin(&self) -> bool {
        self.is_permission("administrator")
    }

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

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct SignInModel {
    #[validate(length(min = 5, max = 15, message = "incorrect"))]
    pub login: String,
    #[validate(length(min = 6, message = "must be 6 characters at least"))]
    pub password: String,
}