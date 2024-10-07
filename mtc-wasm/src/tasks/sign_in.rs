use super::*;

pub fn sign_in_task(event: Event<FormData>) {
    let sync_task = use_coroutine_handle::<SyncAction>();
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let api_client = use_api_client();
    let mut busy = use_busy();

    spawn(async move {
        *busy.write() = true;

        match api_client()
            .post([API_ENDPOINT, API_AUTH].join("/"))
            .json(&json!({
                "login": event.get_str("login"),
                "password": event.get_str("password")
            }))
            .send()
            .await
            .consume()
            .await {
            Ok(_) => {
                sync_task.send(SyncAction::RefreshState("".into()));
                navigator().push(Route::Home {});
            }
            Err(e) =>
                message_box_task.send(MessageBoxAction::Error(e.message())),
        }

        *busy.write() = false;
    });
}