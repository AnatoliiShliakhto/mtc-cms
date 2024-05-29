use axum::async_trait;

use crate::error::db_error::DbError;
use crate::error::Result;
use crate::model::group_model::{GroupCreateModel, GroupModel, GroupsModel, GroupUpdateModel};
use crate::repository::RepositoryPaginate;
use crate::repository_paginate;
use crate::service::group_service::GroupService;

repository_paginate!(GroupService, GroupModel, "groups");

#[async_trait]
pub trait GroupRepositoryTrait {
    async fn find_by_slug(&self, slug: &str) -> Result<GroupModel>;
    async fn find_by_user(&self, login: &str) -> Result<GroupsModel>;
    async fn create(&self, slug: &str, model: GroupCreateModel) -> Result<GroupModel>;
    async fn update(&self, slug: &str, model: GroupUpdateModel) -> Result<GroupModel>;
    async fn delete(&self, slug: &str) -> Result<()>;
}

#[async_trait]
impl GroupRepositoryTrait for GroupService {
    async fn find_by_slug(
        &self,
        slug: &str,
    ) -> Result<GroupModel> {
        self.db.query(r#"
            SELECT * FROM groups WHERE slug=$slug;
            "#)
            .bind(("slug", slug))
            .await?
            .take::<Option<GroupModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn find_by_user(&self, login: &str) -> Result<GroupsModel> {
        self.db.query(r#"
            SELECT array::sort(array::distinct(->user_groups->groups.slug)) as groups FROM users WHERE login=$login
            "#)
            .bind(("login", login))
            .await?
            .take::<Option<GroupsModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn create(
        &self,
        slug: &str,
        model: GroupCreateModel,
    ) -> Result<GroupModel> {
        self.db.query(r#"
            CREATE groups CONTENT {
	            slug: $slug,
	            title: $title
            };
            "#)
            .bind(("slug", slug))
            .bind(("title", model.title))
            .await?
            .take::<Option<GroupModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn update(
        &self,
        slug: &str,
        model: GroupUpdateModel,
    ) -> Result<GroupModel> {
        self.db.query(r#"
            UPDATE groups MERGE {
	            title: $title
            } WHERE slug=$slug;
            "#)
            .bind(("slug", slug))
            .bind(("title", model.title))
            .await?
            .take::<Option<GroupModel>>(0)?
            .ok_or(DbError::EntryUpdate.into())
    }

    async fn delete(
        &self,
        slug: &str,
    ) -> Result<()> {
        self.db.query(r#"
            DELETE FROM groups WHERE slug=$slug;
            "#)
            .bind(("slug", slug))
            .bind(("rel_table", "user_groups"))
            .await?;

        Ok(())
    }
}