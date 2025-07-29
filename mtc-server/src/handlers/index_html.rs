use crate::prelude::{AppState, CONTENT_SECURITY_POLICY, CONTENT_TYPE};
use rand::distr::Alphanumeric;
use rand::Rng;
use std::sync::Arc;

use axum::response::{IntoResponse, Response};

pub async fn get_index_html(state: Arc<AppState>) -> impl IntoResponse {
    let nonce = rand::rng()
        .sample_iter(Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>();
    let csp_header_value = &state
        .config
        .security
        .content_security_policy
        .replace("{{nonce}}", &nonce);
    let index_html = state.template.index_html.replace("{{nonce}}", &nonce);
    Response::builder()
        .header(CONTENT_SECURITY_POLICY, csp_header_value)
        .header(CONTENT_TYPE, "text/html")
        .body(index_html)
        .unwrap()
}
