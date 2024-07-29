use axum::async_trait;
use mtc_model::list_model::{RecordListModel, StringListModel};
use mtc_model::role_model::{RoleCreateModel, RoleModel, RoleUpdateModel};
use mtc_model::record_model::RecordModel;

use crate::error::db_error::DbError;
use crate::error::Result;
use crate::repository::RepositoryPaginate;
use crate::repository_paginate;
use crate::service::role_service::RoleService;

repository_paginate!(RoleService, RoleModel, "roles");

#[async_trait]
pub trait RoleRepositoryTrait {
    async fn all(&self) -> Result<RecordListModel>;
    async fn find_by_slug(&self, slug: &str) -> Result<RoleModel>;
    async fn find_by_user(&self, login: &str) -> Result<StringListModel>;
    async fn create(&self, auth: &str, slug: &str, model: &RoleCreateModel) -> Result<RoleModel>;
    async fn update(&self, auth: &str, slug: &str, model: &RoleUpdateModel) -> Result<RoleModel>;
    async fn delete(&self, slug: &str) -> Result<()>;
    async fn permission_assign(&self, role_id: &str, permission_id: &str) -> Result<()>;
    async fn permissions_drop(&self, role_id: &str) -> Result<()>;
}

#[async_trait]
impl RoleRepositoryTrait for RoleService {
    async fn all(&self) -> Result<RecordListModel> {
        Ok(RecordListModel {
            list: self
                .db
                .query(
                    r#"
                    SELECT slug, title from roles;
                    "#,
                )
                .await?
                .take::<Vec<RecordModel>>(0)?,
        })
    }

    async fn find_by_slug(&self, slug: &str) -> Result<RoleModel> {
        self.db
            .query(
                r#"
                SELECT * FROM roles WHERE slug=$slug;
                "#,
            )
            .bind(("slug", slug))
            .await?
            .take::<Option<RoleModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn find_by_user(&self, login: &str) -> Result<StringListModel> {
        self.db.query(r#"
            SELECT array::sort(array::distinct(->user_roles->roles.slug)) as list FROM users WHERE login=$login
            "#)
            .bind(("login", login))
            .await?
            .take::<Option<StringListModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn create(&self, auth: &str, slug: &str, model: &RoleCreateModel) -> Result<RoleModel> {
        self.db
            .query(
                r#"
                CREATE roles CONTENT {
	                slug: $slug,
	                title: $title,
	                user_access_level: $user_access_level,
	                user_access_all: $user_access_all,
	                created_by: $auth_id,
	                updated_by: $auth_id
                };
                "#,
            )
            .bind(("auth_id", auth))
            .bind(("slug", slug))
            .bind(("title", model.title.clone()))
            .bind(("user_access_level", model.user_access_level))
            .bind(("user_access_all", model.user_access_all))
            .await?
            .take::<Option<RoleModel>>(0)?
            .ok_or(DbError::EntryAlreadyExists.into())
    }

    async fn update(&self, auth: &str, slug: &str, model: &RoleUpdateModel) -> Result<RoleModel> {
        self.db
            .query(
                r#"
                UPDATE roles MERGE {
	                title: $title,
	                user_access_level: $user_access_level,
	                user_access_all: $user_access_all,
	                updated_by: $auth_id
                } WHERE slug=$slug;
                "#,
            )
            .bind(("auth_id", auth))
            .bind(("slug", slug))
            .bind(("title", model.title.clone()))
            .bind(("user_access_level", model.user_access_level))
            .bind(("user_access_all", model.user_access_all))
            .await?
            .take::<Option<RoleModel>>(0)?
            .ok_or(DbError::EntryUpdate.into())
    }

    async fn delete(&self, slug: &str) -> Result<()> {
        self.db
            .query(
                r#"
                DELETE FROM roles WHERE slug=$slug;
                "#,
            )
            .bind(("slug", slug))
            .await?;

        Ok(())
    }

    async fn permission_assign(&self, role_id: &str, permission_id: &str) -> Result<()> {
        match self
            .db
            .query(format!(
                r#"
                RELATE roles:{}->role_permissions->permissions:{};
                "#,
                role_id, permission_id
            ))
            .await
        {
            Ok(..) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    async fn permissions_drop(&self, role_id: &str) -> Result<()> {
        self.db
            .query(
                r#"
                DELETE type::thing('roles', $role_id)->role_permissions;
                "#,
            )
            .bind(("role_id", role_id))
            .await?;

        Ok(())
    }
}
