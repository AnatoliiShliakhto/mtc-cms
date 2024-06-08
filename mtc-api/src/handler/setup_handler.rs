use std::sync::Arc;

use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use axum::extract::State;

use crate::handler::Result;
use crate::model::response_model::ApiResponse;
use crate::state::AppState;

//todo: Make migration service from SQL files or etc
pub async fn setup_handler(state: State<Arc<AppState>>) -> Result<()> {
    let sql = r#"
        BEGIN TRANSACTION;

        REMOVE TABLE IF EXISTS schemas;
        DEFINE TABLE schemas SCHEMAFULL;

        DEFINE FIELD slug ON TABLE schemas TYPE string;
        DEFINE FIELD title ON TABLE schemas TYPE string;
        DEFINE FIELD fields ON TABLE schemas FLEXIBLE TYPE option<array>;
        DEFINE FIELD is_system ON TABLE schemas TYPE bool DEFAULT false;
        DEFINE FIELD is_collection ON TABLE schemas TYPE bool DEFAULT false;
        DEFINE FIELD created_at ON TABLE schemas TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE schemas TYPE datetime VALUE time::now();
        DEFINE INDEX idx_schemas_slug ON TABLE schemas COLUMNS slug UNIQUE;

        CREATE schemas CONTENT {
            slug: 'schemas',
            title: 'Schemas',
            is_system: true
        };

        REMOVE TABLE IF EXISTS sessions;

        CREATE schemas CONTENT {
            slug: 'sessions',
            title: 'Sessions',
            is_system: true
        };

        REMOVE TABLE IF EXISTS users;
        DEFINE TABLE users SCHEMAFULL;

        CREATE schemas CONTENT {
            slug: 'users',
            title: 'Users',
            is_system: true
        };

        DEFINE FIELD login ON TABLE users TYPE string;
        DEFINE FIELD password ON TABLE users TYPE string;
        DEFINE FIELD blocked ON TABLE users TYPE bool DEFAULT false;
        DEFINE FIELD fields ON TABLE users FLEXIBLE TYPE option<object>;
        DEFINE FIELD created_at ON TABLE users TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE users TYPE datetime VALUE time::now();
        DEFINE INDEX idx_users_login ON TABLE users COLUMNS login UNIQUE;

        CREATE users CONTENT {
            id: 'sa',
            login: $login,
            password: $password
        };

        REMOVE TABLE IF EXISTS roles;
        DEFINE TABLE roles SCHEMAFULL;

        CREATE schemas CONTENT {
            slug: 'roles',
            title: 'Roles',
            is_system: true
        };

        DEFINE FIELD slug ON TABLE roles TYPE string;
        DEFINE FIELD title ON TABLE roles TYPE string;
        DEFINE FIELD created_at ON TABLE roles TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE roles TYPE datetime VALUE time::now();
        DEFINE INDEX idx_roles_slug ON TABLE roles COLUMNS slug UNIQUE;

        CREATE roles CONTENT {
            id: 'administrator',
            slug: 'administrator',
            title: 'Administrator'
        };

        CREATE roles CONTENT {
            id: 'anonymous',
            slug: 'anonymous',
            title: 'Anonymous'
        };

        REMOVE TABLE IF EXISTS permissions;
        DEFINE TABLE permissions SCHEMAFULL;

        CREATE schemas CONTENT {
            slug: 'permissions',
            title: 'Permissions',
            is_system: true
        };

        DEFINE FIELD slug ON TABLE permissions TYPE string;
        DEFINE FIELD created_at ON TABLE permissions TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE permissions TYPE datetime VALUE time::now();
        DEFINE INDEX idx_permissions_slug ON TABLE permissions COLUMNS slug UNIQUE;

        CREATE permissions CONTENT {
            id: 'administrator',
            slug: 'administrator'
        };
        CREATE permissions CONTENT {
            id: 'role_read',
            slug: 'role::read'
        };
        CREATE permissions CONTENT {
            id: 'role_write',
            slug: 'role::write'
        };
        CREATE permissions CONTENT {
            id: 'role_delete',
            slug: 'role::delete'
        };
        CREATE permissions CONTENT {
            id: 'group_read',
            slug: 'group::read'
        };
        CREATE permissions CONTENT {
            id: 'group_write',
            slug: 'group::write'
        };
        CREATE permissions CONTENT {
            id: 'group_delete',
            slug: 'group::delete'
        };
        CREATE permissions CONTENT {
            id: 'user_read',
            slug: 'user::read'
        };
        CREATE permissions CONTENT {
            id: 'user_write',
            slug: 'user::write'
        };
        CREATE permissions CONTENT {
            id: 'user_delete',
            slug: 'user::delete'
        };
        CREATE permissions CONTENT {
            id: 'schema_read',
            slug: 'schema::read'
        };
        CREATE permissions CONTENT {
            id: 'schema_write',
            slug: 'schema::write'
        };
        CREATE permissions CONTENT {
            id: 'schema_delete',
            slug: 'schema::delete'
        };

        REMOVE TABLE IF EXISTS role_permissions;
        DEFINE TABLE role_permissions SCHEMAFULL TYPE RELATION IN roles OUT permissions;

        CREATE schemas CONTENT {
            slug: 'role_permissions',
            title: 'Role permissions',
            is_system: true
        };

        DEFINE FIELD created_at ON TABLE role_permissions TYPE datetime VALUE time::now();
        DEFINE INDEX idx_role_permissions ON TABLE role_permissions COLUMNS in, out UNIQUE;

        RELATE roles:administrator->role_permissions->permissions:administrator;
        RELATE roles:administrator->role_permissions->permissions:role_read;
        RELATE roles:administrator->role_permissions->permissions:role_write;
        RELATE roles:administrator->role_permissions->permissions:role_delete;
        RELATE roles:administrator->role_permissions->permissions:group_read;
        RELATE roles:administrator->role_permissions->permissions:group_write;
        RELATE roles:administrator->role_permissions->permissions:group_delete;
        RELATE roles:administrator->role_permissions->permissions:user_read;
        RELATE roles:administrator->role_permissions->permissions:user_write;
        RELATE roles:administrator->role_permissions->permissions:user_delete;
        RELATE roles:administrator->role_permissions->permissions:schema_read;
        RELATE roles:administrator->role_permissions->permissions:schema_write;
        RELATE roles:administrator->role_permissions->permissions:schema_delete;

        REMOVE TABLE IF EXISTS user_roles;
        DEFINE TABLE user_roles SCHEMAFULL TYPE RELATION IN users OUT roles;

        CREATE schemas CONTENT {
            slug: 'user_roles',
            title: 'User roles',
            is_system: true
        };

        DEFINE FIELD created_at ON TABLE user_roles TYPE datetime VALUE time::now();
        DEFINE INDEX idx_user_roles ON TABLE user_roles COLUMNS in, out UNIQUE;

        RELATE users:sa->user_roles->roles:administrator;

        REMOVE TABLE IF EXISTS groups;
        DEFINE TABLE groups SCHEMAFULL;

        CREATE schemas CONTENT {
            slug: 'groups',
            title: 'Groups',
            is_system: true
        };

        DEFINE FIELD slug ON TABLE groups TYPE string;
        DEFINE FIELD title ON TABLE groups TYPE string;
        DEFINE FIELD created_at ON TABLE groups TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE groups TYPE datetime VALUE time::now();
        DEFINE INDEX idx_groups_slug ON TABLE groups COLUMNS slug UNIQUE;

        REMOVE TABLE IF EXISTS user_groups;
        DEFINE TABLE user_groups SCHEMAFULL TYPE RELATION IN users OUT groups;

        CREATE schemas CONTENT {
            slug: 'user_groups',
            title: 'User groups',
            is_system: true
        };

        DEFINE FIELD created_at ON TABLE user_groups TYPE datetime VALUE time::now();
        DEFINE INDEX idx_user_groups ON TABLE user_groups COLUMNS in, out UNIQUE;

        REMOVE TABLE IF EXISTS singles;
        DEFINE TABLE singles SCHEMAFULL;

        CREATE schemas CONTENT {
            slug: 'singles',
            title: 'Singles',
            is_system: true
        };

        DEFINE FIELD slug ON TABLE singles TYPE string;
        DEFINE FIELD fields ON TABLE singles FLEXIBLE TYPE option<object>;
        DEFINE FIELD created_at ON TABLE singles TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE singles TYPE datetime VALUE time::now();
        DEFINE INDEX idx_singles_slug ON TABLE singles COLUMNS slug UNIQUE;

        COMMIT TRANSACTION;
    "#;

    let password = state.cfg.setup_password.clone();
    let salt = SaltString::from_b64(&state.cfg.password_salt).unwrap();

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Error occurred while encrypted password")
        .to_string();

    let responses = state.db.query(sql)
        .bind(("login", state.cfg.setup_login.clone()))
        .bind(("password", password_hash.as_str()))
        .await?;

    println!("{responses:#?}\nInitial migrate done!");

    Ok(ApiResponse::Ok)
}
