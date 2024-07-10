pub mod role_service;
pub mod user_service;
pub mod permissions_service;
pub mod group_service;
pub mod schema_service;
pub mod api_service;
pub mod store_service;
pub mod migration_service;

#[macro_export]
macro_rules! impl_service {
    ($service:ident) => {
        pub struct $service {
            pub cfg: std::sync::Arc<crate::provider::config_provider::Config>,
            pub db: std::sync::Arc<crate::provider::database_provider::Database>,
        }

        impl $service {
            pub fn new(
                cfg: &std::sync::Arc<crate::provider::config_provider::Config>,
                db: &std::sync::Arc<crate::provider::database_provider::Database>,
            ) -> Self {
                Self {
                    cfg: std::sync::Arc::clone(cfg),
                    db: std::sync::Arc::clone(db),
                }
            }
        }
    }
}
