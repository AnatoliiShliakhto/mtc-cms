use dioxus::prelude::*;

use mtc_model::auth_model::AuthModel;

use crate::state::AppState;

pub static APP: GlobalSignal<AppState> = Signal::global(|| AppState::new());
pub static APP_ERROR: GlobalSignal<String> = Signal::global(|| String::new());
pub static APP_AUTH: GlobalSignal<AuthModel> = Signal::global(|| AuthModel {
    id: "anonymous".to_string(),
    roles: vec![],
    groups: vec![],
    permissions: vec![],
});