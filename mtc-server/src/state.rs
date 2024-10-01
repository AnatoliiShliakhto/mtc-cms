use crate::prelude::*;

pub struct AppState {
    pub config: Arc<Config>,
    pub repository: Repository,
}

impl AppState {
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