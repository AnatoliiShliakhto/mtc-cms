use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::model::response_model::ApiResponse;
use crate::provider::config_provider::CFG;
use crate::provider::database_provider::DB;

//todo: Make migration service from SQL files or etc
pub async fn setup_handler() -> Result<ApiResponse<()>, ApiError> {
    let sql = r#"
        REMOVE TABLE users;
        REMOVE TABLE user_role;
        REMOVE TABLE roles;
        REMOVE TABLE role_permissions;
        REMOVE TABLE permissions;
        REMOVE TABLE fields;
        REMOVE TABLE single_type;

        BEGIN TRANSACTION;

        DEFINE TABLE users;

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

        DEFINE TABLE roles;

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

        DEFINE TABLE permissions;

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

        DEFINE TABLE role_permissions;

        DEFINE FIELD created_at ON TABLE permissions TYPE datetime VALUE time::now();

        RELATE roles:administrator->role_permissions->permissions:roles_read;
        RELATE roles:administrator->role_permissions->permissions:roles_write;
        RELATE roles:administrator->role_permissions->permissions:roles_delete;

        DEFINE TABLE user_roles;

        DEFINE FIELD created_at ON TABLE user_role TYPE datetime VALUE time::now();

        RELATE users:sa->user_roles->roles:administrator;

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
        .await
        .map_err(|e| DbError::SomethingWentWrong(e.to_string()));

    println!("{responses:?}");
    println!();
    println!("Initial migrate done!");

    Ok(ApiResponse::Ok)
}
