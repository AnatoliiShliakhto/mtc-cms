use std::sync::Arc;

use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use axum::extract::State;

use crate::error::Result;
use crate::model::response_model::ApiResponse;
use crate::state::AppState;

//todo: Make migration service from SQL files or etc
pub async fn setup_handler(state: State<Arc<AppState>>) -> Result<ApiResponse<()>> {
    let sql = r#"
        BEGIN TRANSACTION;

        REMOVE TABLE IF EXISTS schemas;
        DEFINE TABLE schemas SCHEMAFULL;

        DEFINE FIELD name ON TABLE schemas TYPE string;
        DEFINE FIELD fields ON TABLE schemas TYPE option<array>;
        DEFINE FIELD is_system ON TABLE schemas TYPE bool DEFAULT false;
        DEFINE FIELD is_collection ON TABLE schemas TYPE bool DEFAULT false;
        DEFINE FIELD created_at ON TABLE schemas TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE schemas TYPE datetime VALUE time::now();
        DEFINE INDEX idx_schemas_name ON TABLE schemas COLUMNS name UNIQUE;

        CREATE schemas CONTENT {
            name: 'schemas',
            is_system: true
        };

        REMOVE TABLE IF EXISTS sessions;

        CREATE schemas CONTENT {
            name: 'sessions',
            is_system: true
        };

        REMOVE TABLE IF EXISTS users;
        DEFINE TABLE users SCHEMAFULL;

        CREATE schemas CONTENT {
            name: 'users',
            is_system: true
        };

        DEFINE FIELD login ON TABLE users TYPE string;
        DEFINE FIELD password ON TABLE users TYPE string;
        DEFINE FIELD blocked ON TABLE users TYPE bool DEFAULT false;
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
            name: 'roles',
            is_system: true
        };

        DEFINE FIELD name ON TABLE roles TYPE string;
        DEFINE FIELD title ON TABLE roles TYPE string;
        DEFINE FIELD created_at ON TABLE roles TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE roles TYPE datetime VALUE time::now();
        DEFINE INDEX idx_roles_name ON TABLE roles COLUMNS name UNIQUE;

        CREATE roles CONTENT {
            id: 'administrator',
            name: 'administrator',
            title: 'Адміністратор'
        };

        CREATE roles CONTENT {
            id: 'anonymous',
            name: 'anonymous',
            title: 'Анонім'
        };

        REMOVE TABLE IF EXISTS permissions;
        DEFINE TABLE permissions SCHEMAFULL;

        CREATE schemas CONTENT {
            name: 'permissions',
            is_system: true
        };

        DEFINE FIELD name ON TABLE permissions TYPE string;
        DEFINE FIELD created_at ON TABLE permissions TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE permissions TYPE datetime VALUE time::now();
        DEFINE INDEX idx_permissions_name ON TABLE permissions COLUMNS name UNIQUE;

        CREATE permissions CONTENT {
            id: 'roles_read',
            name: 'roles::read'
        };
        CREATE permissions CONTENT {
            id: 'roles_write',
            name: 'roles::write'
        };
        CREATE permissions CONTENT {
            id: 'roles_delete',
            name: 'roles::delete'
        };
        CREATE permissions CONTENT {
            id: 'groups_read',
            name: 'groups::read'
        };
        CREATE permissions CONTENT {
            id: 'groups_write',
            name: 'groups::write'
        };
        CREATE permissions CONTENT {
            id: 'groups_delete',
            name: 'groups::delete'
        };
        CREATE permissions CONTENT {
            id: 'users_read',
            name: 'users::read'
        };
        CREATE permissions CONTENT {
            id: 'users_write',
            name: 'users::write'
        };
        CREATE permissions CONTENT {
            id: 'users_delete',
            name: 'users::delete'
        };
        CREATE permissions CONTENT {
            id: 'schemas_read',
            name: 'schemas::read'
        };
        CREATE permissions CONTENT {
            id: 'schemas_write',
            name: 'schemas::write'
        };
        CREATE permissions CONTENT {
            id: 'schemas_delete',
            name: 'schemas::delete'
        };

        REMOVE TABLE IF EXISTS role_permissions;
        DEFINE TABLE role_permissions SCHEMAFULL TYPE RELATION IN roles OUT permissions;

        CREATE schemas CONTENT {
            name: 'role_permissions',
            is_system: true
        };

        DEFINE FIELD created_at ON TABLE role_permissions TYPE datetime VALUE time::now();
        DEFINE INDEX idx_role_permissions ON TABLE role_permissions COLUMNS in, out UNIQUE;

        RELATE roles:administrator->role_permissions->permissions:roles_read;
        RELATE roles:administrator->role_permissions->permissions:roles_write;
        RELATE roles:administrator->role_permissions->permissions:roles_delete;
        RELATE roles:administrator->role_permissions->permissions:groups_read;
        RELATE roles:administrator->role_permissions->permissions:groups_write;
        RELATE roles:administrator->role_permissions->permissions:groups_delete;
        RELATE roles:administrator->role_permissions->permissions:users_read;
        RELATE roles:administrator->role_permissions->permissions:users_write;
        RELATE roles:administrator->role_permissions->permissions:users_delete;
        RELATE roles:administrator->role_permissions->permissions:schemas_read;
        RELATE roles:administrator->role_permissions->permissions:schemas_write;
        RELATE roles:administrator->role_permissions->permissions:schemas_delete;

        REMOVE TABLE IF EXISTS user_roles;
        DEFINE TABLE user_roles SCHEMAFULL TYPE RELATION IN users OUT roles;

        CREATE schemas CONTENT {
            name: 'user_roles',
            is_system: true
        };

        DEFINE FIELD created_at ON TABLE user_roles TYPE datetime VALUE time::now();
        DEFINE INDEX idx_user_roles ON TABLE user_roles COLUMNS in, out UNIQUE;

        RELATE users:sa->user_roles->roles:administrator;

        REMOVE TABLE IF EXISTS groups;
        DEFINE TABLE groups SCHEMAFULL;

        CREATE schemas CONTENT {
            name: 'groups',
            is_system: true
        };

        DEFINE FIELD name ON TABLE groups TYPE string;
        DEFINE FIELD title ON TABLE groups TYPE string;
        DEFINE FIELD created_at ON TABLE groups TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE groups TYPE datetime VALUE time::now();
        DEFINE INDEX idx_groups_name ON TABLE groups COLUMNS name UNIQUE;

        REMOVE TABLE IF EXISTS user_groups;
        DEFINE TABLE user_groups SCHEMAFULL TYPE RELATION IN users OUT groups;

        CREATE schemas CONTENT {
            name: 'user_groups',
            is_system: true
        };

        DEFINE FIELD created_at ON TABLE user_groups TYPE datetime VALUE time::now();
        DEFINE INDEX idx_user_groups ON TABLE user_groups COLUMNS in, out UNIQUE;

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
