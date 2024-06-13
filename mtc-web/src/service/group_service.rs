use dioxus::hooks::UnboundedReceiver;
use dioxus::prelude::*;
use futures_util::StreamExt;

use crate::action::group_action::GroupAction;
use crate::global_signal::APP;
use crate::handler::group_handler::GroupHandler;
use crate::service::assign_error;

pub async fn group_service(mut rx: UnboundedReceiver<GroupAction>) {
    let app_state = &*APP.read_unchecked();

    while let Some(msg) = rx.next().await {
        match msg {
            GroupAction::Page(num) => {
                app_state.get_group_list(num)
                    .await
                    .map_err(|e| assign_error(e))
                    .map(|res| {})
                    .unwrap_or(())
            }
        }
    }
}
