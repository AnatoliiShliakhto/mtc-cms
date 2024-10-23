use super::*;

pub enum SyncAction {
    RefreshState(Cow<'static, str>),
}

pub async fn sync_service(mut rx: UnboundedReceiver<SyncAction>) {
    let mut auth_state = use_auth_state();
    let mut search_list = use_search_engine_list();
    let mut search_idx = use_search_engine_index();
    let api_client = use_api_client();
    let app_state = use_app_state();
    let mut state_roles = app_state.roles;
    let mut state_groups = app_state.groups;

    while let Some(msg) = rx.next().await {
        match msg {
            SyncAction::RefreshState(auth_id) => {
                let Ok(response) = api_client()
                    .post([API_ENDPOINT, "sync"].join("/"))
                    .json(&json!({"id": auth_id}))
                    .send()
                    .await else { continue};

                let Ok(value) = response.json::<Value>().await else { continue };
                if let Some(auth) = value.key_obj::<AuthState>("auth") {
                    *auth_state.write() = auth;
                }
                if let Some(pages) = value.key_obj::<Vec<Entry>>("pages") {
                    *use_pages_entries().write() = pages;
                }
                if let Some(roles) = value.key_obj::<Vec<Entry>>("roles") {
                    state_roles.set(roles)
                }
                if let Some(groups) = value.key_obj::<Vec<Entry>>("groups") {
                    state_groups.set(groups)
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