use super::*;

pub enum SyncAction {
    RefreshState(),
}

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