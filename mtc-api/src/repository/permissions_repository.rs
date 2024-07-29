use axum::async_trait;

use mtc_model::list_model::{RecordListModel, StringListModel};
use mtc_model::permission_model::PermissionModel;
use mtc_model::record_model::RecordModel;

use crate::error::db_error::DbError;
use crate::error::Result;
use crate::service::permissions_service::PermissionsService;

#[async_trait]
pub trait PermissionsRepositoryTrait {
    async fn all(&self) -> Result<RecordListModel>;
    async fn find_by_slug(&self, slug: &str) -> Result<PermissionModel>;
    async fn find_by_role(&self, role: &str) -> Result<StringListModel>;
    async fn find_by_user(&self, role: &str) -> Result<StringListModel>;
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
}
