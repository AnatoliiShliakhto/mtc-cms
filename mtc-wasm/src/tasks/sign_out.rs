use super::*;

pub fn sign_out_task(event: Event<MouseData>) {
    let sync_task = use_coroutine_handle::<SyncAction>();
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let api_client = use_api_client();

    spawn(async move {
        match api_client()
            .delete([API_ENDPOINT, API_AUTH].join("/"))
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