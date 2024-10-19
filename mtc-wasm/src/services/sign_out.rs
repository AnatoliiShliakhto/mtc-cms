use super::*;

pub fn sign_out_task(event: Event<MouseData>) {
    let sync = use_coroutine_handle::<SyncAction>();

    spawn(async move {
        if delete_request!(url!(API_AUTH)) {
            sync.send(SyncAction::RefreshState("".into()));

            if navigator().can_go_back() {
                navigator().go_back()
            } else {
                navigator().replace(route!());
            }
        }
    });
}