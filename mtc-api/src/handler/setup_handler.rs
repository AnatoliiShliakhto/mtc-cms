use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;

use crate::error::Result;
use crate::model::response_model::ApiResponse;
use crate::provider::config_provider::CFG;
use crate::provider::database_provider::DB;

//todo: Make migration service from SQL files or etc
pub async fn setup_handler() -> Result<ApiResponse<()>> {
    let sql = r#"
        BEGIN TRANSACTION;

        REMOVE TABLE IF EXISTS tables;
        DEFINE TABLE tables SCHEMAFULL;

        DEFINE FIELD name ON TABLE tables TYPE string;
        DEFINE FIELD is_core ON TABLE tables TYPE bool DEFAULT false;
        DEFINE FIELD created_at ON TABLE tables TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE tables TYPE datetime VALUE time::now();
        DEFINE INDEX idx_tables_name ON TABLE tables COLUMNS name UNIQUE;

        CREATE tables CONTENT {
            name: 'tables',
            is_core: true
        };

        REMOVE TABLE IF EXISTS sessions;

        CREATE tables CONTENT {
            name: 'sessions',
            is_core: true
        };

        REMOVE TABLE IF EXISTS users;
        DEFINE TABLE users SCHEMAFULL;

        CREATE tables CONTENT {
            name: 'users',
            is_core: true
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

        CREATE tables CONTENT {
            name: 'roles',
            is_core: true
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

        CREATE tables CONTENT {
            name: 'permissions',
            is_core: true
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
            id: 'permissions_read',
            name: 'permissions::read'
        };
        CREATE permissions CONTENT {
            id: 'permissions_write',
            name: 'permissions::write'
        };
        CREATE permissions CONTENT {
            id: 'permissions_delete',
            name: 'permissions::delete'
        };

        REMOVE TABLE IF EXISTS role_permissions;
        DEFINE TABLE role_permissions SCHEMAFULL TYPE RELATION IN roles OUT permissions;

        CREATE tables CONTENT {
            name: 'role_permissions',
            is_core: true
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
        RELATE roles:administrator->role_permissions->permissions:permissions_read;
        RELATE roles:administrator->role_permissions->permissions:permissions_write;
        RELATE roles:administrator->role_permissions->permissions:permissions_delete;

        REMOVE TABLE IF EXISTS user_roles;
        DEFINE TABLE user_roles SCHEMAFULL TYPE RELATION IN users OUT roles;

        CREATE tables CONTENT {
            name: 'user_roles',
            is_core: true
        };

        DEFINE FIELD created_at ON TABLE user_roles TYPE datetime VALUE time::now();
        DEFINE INDEX idx_user_roles ON TABLE user_roles COLUMNS in, out UNIQUE;

        RELATE users:sa->user_roles->roles:administrator;

        REMOVE TABLE IF EXISTS groups;
        DEFINE TABLE groups SCHEMAFULL;

        CREATE tables CONTENT {
            name: 'groups',
            is_core: true
        };

        DEFINE FIELD name ON TABLE groups TYPE string;
        DEFINE FIELD title ON TABLE groups TYPE string;
        DEFINE FIELD created_at ON TABLE groups TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE groups TYPE datetime VALUE time::now();
        DEFINE INDEX idx_groups_name ON TABLE groups COLUMNS name UNIQUE;

        REMOVE TABLE IF EXISTS user_groups;
        DEFINE TABLE user_groups SCHEMAFULL TYPE RELATION IN users OUT groups;

        CREATE tables CONTENT {
            name: 'user_groups',
            is_core: true
        };

        DEFINE FIELD created_at ON TABLE user_groups TYPE datetime VALUE time::now();
        DEFINE INDEX idx_user_groups ON TABLE user_groups COLUMNS in, out UNIQUE;

        REMOVE TABLE IF EXISTS single_types;
        DEFINE TABLE single_types SCHEMAFULL;

        CREATE tables CONTENT {
            name: 'single_types',
            is_core: true
        };

        DEFINE FIELD api ON TABLE single_types TYPE string;
        DEFINE FIELD fields ON TABLE single_types TYPE array;
        DEFINE FIELD created_at ON TABLE single_types TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE single_types TYPE datetime VALUE time::now();
        DEFINE INDEX idx_single_types_api ON TABLE single_types COLUMNS api UNIQUE;

        COMMIT TRANSACTION;
    "#;

    let password = CFG.setup_password.clone();
    let salt = SaltString::from_b64(&CFG.password_salt).unwrap();

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Error occurred while encrypted password")
        .to_string();

    let responses = DB.query(sql)
        .bind(("login", CFG.setup_login.clone()))
        .bind(("password", password_hash.as_str()))
        .await?;

    println!("{responses:?}");
    println!();
    println!("Initial migrate done!");

    Ok(ApiResponse::Ok)
}
