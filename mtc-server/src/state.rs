use crate::prelude::*;

pub struct AppState {
    pub config: Arc<Config>,
    pub repository: Repository,
    pub smtp_client: Arc<SmtpClient>,
    pub template: Arc<Template>,
}

impl AppState {
    pub fn init(cfg: Config, db: Database, smtp_client: SmtpClient, template: Template) -> Self {
        let config = Arc::new(cfg);
        let database = Arc::new(db);
        let repository = Repository::init(&database, &config);
        let smtp_client = Arc::new(smtp_client);
        let template = Arc::new(template);

        Self {
            config,
            repository,
            smtp_client,
            template,
        }
    }
}
