use dioxus::prelude::*;

use mtc_model::auth_model::AuthModel;

use crate::handler::ApiHandler;
use crate::model::modal_model::ModalModel;
use crate::service::AppService;

pub struct AppState {
    pub api: ApiHandler,
    pub service: AppService,
    pub auth: GlobalSignal<AuthModel>,
    pub modal: GlobalSignal<ModalModel>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            api: ApiHandler::new(),
            service: AppService,
            auth: Signal::global(AuthModel::default),
            modal: Signal::global(|| ModalModel::None),
        }
    }
}