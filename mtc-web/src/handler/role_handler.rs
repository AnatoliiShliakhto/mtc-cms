use mtc_model::permission_model::PermissionsModel;
use mtc_model::role_model::{RoleCreateModel, RoleModel, RolesModel, RoleUpdateModel};

use crate::error::api_error::ApiError;
use crate::handler::{ApiHandler, HandlerResponse};
use crate::model::response_model::ApiResponse;

pub trait RoleHandler {
    async fn get_role_all(&self) -> Result<RolesModel, ApiError>;
    async fn get_role_list(&self, page: usize) -> Result<ApiResponse<Vec<RoleModel>>, ApiError>;
    async fn delete_role(&self, slug: &str) -> Result<(), ApiError>;
    async fn delete_role_list(&self, roles: RolesModel) -> Result<(), ApiError>;
    async fn create_role(&self, slug: &str, group: &RoleCreateModel) -> Result<RoleModel, ApiError>;
    async fn update_role(&self, slug: &str, group: &RoleUpdateModel) -> Result<RoleModel, ApiError>;
    async fn get_role_permissions(&self, slug: &str) -> Result<PermissionsModel, ApiError>;
}

impl RoleHandler for ApiHandler {
    async fn get_role_all(&self) -> Result<RolesModel, ApiError> {
        self
            .api_client
            .get([&self.api_url, "role", "all"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }
    
    async fn get_role_list(&self, page: usize) -> Result<ApiResponse<Vec<RoleModel>>, ApiError> {
        self
            .api_client
            .get([&self.api_url, "role", "list", &page.to_string()].join("/"))
            .send()
            .await
            .consume_page()
            .await
    }

    async fn delete_role(&self, slug: &str) -> Result<(), ApiError> {
        self
            .api_client
            .delete([&self.api_url, "role", slug].join("/"))
            .send()
            .await?;
        Ok(())
    }

    async fn delete_role_list(&self, roles: RolesModel) -> Result<(), ApiError> {
        self
            .api_client
            .delete([&self.api_url, "role", "list"].join("/"))
            .json(&roles)
            .send()
            .await?;
        Ok(())
    }

    async fn create_role(&self, slug: &str, role: &RoleCreateModel) -> Result<RoleModel, ApiError> {
        self
            .api_client
            .post([&self.api_url, "role", slug].join("/"))
            .json(role)
            .send()
            .await
            .consume_data()
            .await
    }

    async fn update_role(&self, slug: &str, role: &RoleUpdateModel) -> Result<RoleModel, ApiError> {
        self
            .api_client
            .patch([&self.api_url, "role", slug].join("/"))
            .json(role)
            .send()
            .await
            .consume_data()
            .await
    }

    async fn get_role_permissions(&self, slug: &str) -> Result<PermissionsModel, ApiError> {
        self
            .api_client
            .get([&self.api_url, "role", slug, "permissions"].join("/"))
            .send()
            .await
            .consume_data()
            .await
    }
}