use super::*;

pub fn sign_out_task(event: Event<MouseData>) {
    let sync_task = use_coroutine_handle::<SyncAction>();
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();

    spawn(async move {
        match use_api_client()
            .delete([API_URL, "auth"].join("/"))
            .send()
            .await
            .consume()
            .await {
            Ok(_) =>
                sync_task.send(SyncAction::RefreshState("".into())),
            Err(e) =>
                message_box_task.send(MessageBoxAction::Error(e.message())),
        }
    });
}