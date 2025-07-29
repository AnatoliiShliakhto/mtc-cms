use crate::prelude::*;

pub struct AppState {
    pub config: Arc<Config>,
    pub repository: Repository,
    pub template: Arc<Template>,
}

impl AppState {
    pub fn init(cfg: Config, db: Database, template: Template) -> Self {
        let config = Arc::new(cfg);
        let database = Arc::new(db);
        let repository = Repository::init(&database, &config);
        let template = Arc::new(template);

        Self {
            config,
            repository,
            template,
        }
    }
}
