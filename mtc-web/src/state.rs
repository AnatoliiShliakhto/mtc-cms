use dioxus::prelude::*;

use mtc_model::auth_model::AuthModel;

use crate::handler::ApiHandler;
use crate::service::AppService;

pub struct AppState {
    pub api: ApiHandler,
    pub service: AppService,
    pub auth: GlobalSignal<AuthModel>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            api: ApiHandler::new(),
            service: AppService,
            auth: Signal::global(|| AuthModel::default()),
        }
    }
}