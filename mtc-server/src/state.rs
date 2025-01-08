use crate::prelude::*;

pub struct AppState {
    pub config: Arc<Config>,
    pub repository: Repository,
}

impl AppState {
    /// Initializes a new instance of the `AppState` with the given
    /// configuration and database connection.
    ///
    /// # Arguments
    ///
    /// * `cfg` - The configuration of the application.
    /// * `db` - The database connection to initialize the repository with.
    ///
    /// # Returns
    ///
    /// A new `AppState` with the initialized repository and configuration.
    pub fn init(cfg: Config, db: Database) -> Self {
        let config = Arc::new(cfg);
        let database = Arc::new(db);

        let repository = Repository::init(&database, &config);

        Self {
            config,
            repository,
        }
    }
}    