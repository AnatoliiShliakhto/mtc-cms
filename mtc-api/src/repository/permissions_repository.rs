use axum::async_trait;

use mtc_model::list_model::{RecordListModel, StringListModel};
use mtc_model::permission_model::{PermissionDtoModel, PermissionModel};
use mtc_model::record_model::RecordModel;
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::Result;
use crate::service::permissions_service::PermissionsService;

#[async_trait]
pub trait PermissionsRepositoryTrait {
    async fn all(&self) -> Result<RecordListModel>;
    async fn find_by_slug(&self, slug: &str) -> Result<PermissionModel>;
    async fn find_by_role(&self, role: &str) -> Result<StringListModel>;
    async fn find_by_user(&self, role: &str) -> Result<StringListModel>;
    async fn get_custom(&self) -> Result<Vec<PermissionModel>>;
    async fn create_custom(&self, auth: &str, model: PermissionDtoModel) -> Result<()>;
    async fn delete_custom(&self, model: PermissionDtoModel) -> Result<()>;
}

#[async_trait]
impl PermissionsRepositoryTrait for PermissionsService {
    async fn all(&self) -> Result<RecordListModel> {
        Ok(RecordListModel {
            list: self
                .db
                .query(
                    r#"
                SELECT slug, slug as title FROM permissions;
                "#,
                )
                .await?
                .take::<Vec<RecordModel>>(0)?,
        })
    }

    async fn find_by_slug(&self, slug: &str) -> Result<PermissionModel> {
        self.db
            .query(
                r#"
            SELECT * FROM permissions WHERE slug=$slug;
            "#,
            )
            .bind(("slug", slug))
            .await?
            .take::<Option<PermissionModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn find_by_role(&self, slug: &str) -> Result<StringListModel> {
        self.db.query(r#"
            SELECT array::sort(array::distinct(->role_permissions->permissions.slug)) as list FROM roles WHERE slug=$slug
            "#)
            .bind(("slug", slug))
            .await?
            .take::<Option<StringListModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn find_by_user(&self, login: &str) -> Result<StringListModel> {
        self.db.query(r#"
            SELECT array::sort(array::distinct(->user_roles->roles->role_permissions->permissions.slug)) as list
            FROM users WHERE login=$login
            "#)
            .bind(("login", login))
            .await?
            .take::<Option<StringListModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn get_custom(&self) -> Result<Vec<PermissionModel>> {
        Ok(self.db.query(r#"
            SELECT * FROM permissions WHERE is_custom = true;
            "#)
            .await?
            .take::<Vec<PermissionModel>>(0)?)
    }

    async fn create_custom(&self, auth: &str, model: PermissionDtoModel) -> Result<()> {
        let permission = self.db
            .query(
                r#"
                CREATE permissions CONTENT {
	                slug: $slug,
	                is_custom: true,
	                created_by: $auth_id
                };
            "#,
            )
            .bind(("auth_id", auth))
            .bind(("slug", model.slug))
            .await?
            .take::<Option<PermissionModel>>(0)?;
        
        match permission {
            Some(value) => {
                self.db
                    .query(format!(
                        r#"
                        RELATE roles:administrator->role_permissions->permissions:{0};
                        "#,
                        value.id
                    ))
                    .await?;
            },
            _ => Err(ApiError::from(DbError::EntryNotFound))?,
        }
        
        Ok(())    
    }

    async fn delete_custom(&self, model: PermissionDtoModel) -> Result<()> {
        self.db
            .query(
                r#"
                DELETE FROM permissions WHERE slug=$slug and is_custom = true;
                "#,
            )
            .bind(("slug", model.slug))
            .await?;

        Ok(())
    }
}
