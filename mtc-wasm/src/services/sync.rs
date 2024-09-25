use super::*;

pub enum SyncAction {
    RefreshState(Cow<'static, str>),
}

pub async fn sync_service(mut rx: UnboundedReceiver<SyncAction>) {
    let mut auth_state = use_auth_state();
    let mut search_list = use_search_engine_list();
    let mut search_idx = use_search_engine_index();

    while let Some(msg) = rx.next().await {
        match msg {
            SyncAction::RefreshState(auth_id) => {
                let Ok(response) = use_api_client()
                    .post([API_URL, "sync"].join("/"))
                    .json(&json!({"id": auth_id}))
                    .send()
                    .await
                    .consume()
                    .await else { continue };

                let Ok(value) = response.json::<Value>().await else { continue };
                if let Some(auth) = value.get_object::<AuthState>("auth") {
                    *auth_state.write() = auth;
                }
                if let Some(search) =
                    value.get_object::<Vec<SearchIdxDto>>("search_idx") {
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