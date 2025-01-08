use super::*;

pub enum SyncAction {
    RefreshState(),
}

/// Synchronize the state of the application.
///
/// This function is called when the application is initialized or when the user
/// logs in or out. It is responsible for refreshing the state of the application
/// by sending a request to the server and updating the state of the application
/// accordingly.
///
/// The function takes a receiver of [`SyncAction`] messages, which are sent to it
/// whenever the state of the application needs to be refreshed.
///
/// The function is an infinite loop that runs until the receiver is closed. It
/// receives messages from the receiver and processes them accordingly. If the
/// message is [`SyncAction::RefreshState`], it sends a request to the server and
/// updates the state of the application accordingly.
pub async fn sync_service(mut rx: UnboundedReceiver<SyncAction>) {
    let mut search_list = state_fn!(search_engine).list;
    let mut search_idx = state_fn!(search_engine).index;

    while let Some(msg) = rx.next().await {
        match msg {
            SyncAction::RefreshState() => {
                let Ok(response) =  state!(client)
                    .get([API_ENDPOINT, "sync"].join("/"))
                    .send()
                    .await else { continue};

                let Ok(value) = response.json::<Value>().await else { continue };
                if let Some(auth) = value.key_obj::<AuthState>("auth") {
                    state!(set_auth, auth)
                }
                if let Some(pages) = value.key_obj::<Vec<Entry>>("pages") {
                    state!(set_pages, pages)
                }
                if let Some(roles) = value.key_obj::<Vec<Entry>>("roles") {
                    state!(set_roles, roles)
                }
                if let Some(groups) = value.key_obj::<Vec<Entry>>("groups") {
                    state!(set_groups, groups)
                }
                if let Some(search) =
                    value.key_obj::<Vec<SearchIdxDto>>("search_idx") {
                    let mut new_search_idx = simsearch::SimSearch::new();
                    search_list.write().clear();

                    for (count, item) in search.iter().enumerate() {
                        new_search_idx.insert(count, &item.title);
                        search_list.write().insert(count, item.to_owned());
                    }

                    *search_idx.write() = new_search_idx
                }
            }
        }
    }
}