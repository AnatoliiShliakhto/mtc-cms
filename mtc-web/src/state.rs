use std::collections::BTreeMap;
use dioxus::prelude::*;
use mtc_model::auth_model::AuthModel;
use mtc_model::record_model::RecordModel;
use mtc_model::user_details_model::UserDetailsModel;
use crate::handler::ApiHandler;
use crate::model::modal_model::ModalModel;
use crate::service::AppService;

pub struct AppState {
    pub api: ApiHandler,
    pub service: AppService,
    pub auth: GlobalSignal<AuthModel>,
    pub modal: GlobalSignal<ModalModel>,
    pub users: GlobalSignal<BTreeMap<String, UserDetailsModel>>,
    pub active_content_api: GlobalSignal<RecordModel>,
    pub active_content: GlobalSignal<RecordModel>,
    pub is_busy: GlobalSignal<bool>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            api: ApiHandler::default(),
            service: AppService,
            auth: Signal::global(AuthModel::default),
            modal: Signal::global(|| ModalModel::None),
            users: Signal::global(BTreeMap::<String, UserDetailsModel>::new),
            active_content_api: Signal::global(RecordModel::default),
            active_content: Signal::global(RecordModel::default),
            is_busy: Signal::global(|| false),
        }
    }
}