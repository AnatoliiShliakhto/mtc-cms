use super::*;
use wasm_bindgen_futures::JsFuture;

/// The component renders a panel on the right side of the screen with
/// several buttons to perform actions on the personnel table.
///
/// The buttons are:
///
/// - Sync: Retrieves the list of personnel from the server.
/// - Add: Opens the form to add a new personnel entry.
/// - Paste from Clipboard: Copies the list of personnel from the clipboard.
/// - Copy to Clipboard: Copies the list of personnel to the clipboard.
/// - Upload: Opens a file dialog to select a JSON file with personnel data.
/// - Export: Exports the personnel data to a JSON file.
/// - Clear: Clears the personnel table.
#[component]
pub fn PersonnelActions() -> Element {
    let personnel_import_element_id = "personnel-import";
    let mut users = state_fn!(personnel);
    let PersonnelColumns {
        actions: column_actions,
        login: column_login,
        rank: column_rank,
        name: column_name,
        password: column_password,
        group: column_group,
        access: column_access,
    } = state_fn!(personnel_columns);

    let from_clipboard = move |_| async move {
        match JsFuture::from(jsFfiCopyFromClipboard())
            .await
            .ok()
            .and_then(|jsValue| jsValue.as_string())
        {
            Some(text) => {
                let mut reader = csv::ReaderBuilder::new()
                    .delimiter(b'\t')
                    .has_headers(false)
                    .flexible(true)
                    .trim(csv::Trim::All)
                    .from_reader(text.as_bytes());

                users.write().clear();
                for item in reader.deserialize::<PersonDto>() {
                    if let Ok(item) = item {
                        let login = item.login.replace(" ", "").to_uppercase();
                        users.write().insert(
                            login.clone().into(),
                            UserDetails {
                                login: login.into(),
                                rank: item.rank.trim().to_string().to_lowercase().into(),
                                name: item.name.trim().to_string().into(),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
            None => error!("failed to invoke jsFfiCopyFromClipboard")
        }
    };

    let to_clipboard = move |_| async move {
        let payload = Value::String(
            std::iter::Map::collect::<Vec<String>>(users().iter().map(|(login, details)| {
                let mut array = vec![];
                if column_login() {
                    array.push(details.login.clone());
                }
                if column_rank() {
                    array.push(details.rank.clone());
                }
                if column_name() {
                    array.push(details.name.clone());
                }
                if column_password() {
                    array.push(details.password.clone());
                }
                if column_group() {
                    array.push(details.group.clone());
                }
                if column_access() {
                    if let Some(access) = &details.last_access {
                        array.push(
                            format!(
                                "{} ({})",
                                access
                                    .parse::<DateTime<Local>>()
                                    .unwrap_or_default()
                                    .format("%d/%m/%Y")
                                    .to_string(),
                                &details.access_count
                            )
                            .into(),
                        )
                    } else {
                        array.push("".into());
                    }
                }

                array.join("\t")
            }))
            .join("\r\n"),
        );

        match JsFuture::from(jsFfiCopyToClipboard(payload.as_str().unwrap())).await {
            Ok(_) => success_dialog!("message-personnel-copy-successful"),
            Err(_) => error!("failed to invoke jsFfiCopyToClipboard"),
        }
    };

    let from_file = move |event: Event<FormData>| async move {
        let files = event.files();
        if files.is_empty() {
            return;
        }
        if let Some(file) = files.into_iter().next() {
            if let Ok(json_string) = file.read_string().await {
                let Ok(personnel) = serde_json::from_str::<Vec<PersonDto>>(&json_string) else {
                    return;
                };

                users.write().clear();
                personnel.iter().for_each(|item| {
                    users.write().insert(
                        item.login.clone(),
                        UserDetails {
                            login: item.login.clone(),
                            rank: item.rank.clone(),
                            name: item.name.clone(),
                            ..Default::default()
                        },
                    );
                });
            }

            if JsFuture::from(jsFfiClearFileInput()).await.is_err() {
                error!("failed to invoke jsFfiClearFileInput");
            }
        }
    };

    let to_file = move |_| async move {
        let payload = users()
            .iter()
            .map(|(login, details)| PersonDto {
                login: details.login.clone(),
                rank: details.rank.clone(),
                name: details.name.clone(),
            })
            .collect::<Vec<PersonDto>>();

        match serde_json::to_string(payload.as_slice()) {
            Ok(json_str) => {
                if JsFuture::from(jsFfiExportJsonFile(&json_str, "mtc-users.json")).await.is_err() {
                    error!("failed to invoke jsFfiExportJsonFile");
                }
            }
            Err(error) => {
                error!("failed to serialize to JSON: {}", error);
            }
        }
    };

    let personnel_check = move |_| {
        let payload = json!(
            users()
                .iter()
                .map(|(login, _)| login.clone())
                .collect::<Vec<Cow<'static, str>>>()
        );

        spawn(async move {
            let response = value_request!(url!(API_PERSONNEL), payload);
            let Some(user_details_dto) = response.self_obj::<Vec<UserDetailsDto>>() else {
                return;
            };
            state!(add_personnel, user_details_dto);
        });
    };

    rsx! {
        div {
            class: "action-right-panel group join join-vertical top-40",
            class: "opacity-50 xl:opacity-100 hover:opacity-100",
            onclick: move |event| event.stop_propagation(),

            button {
                class: "hover:btn-primary join-item",
                onclick: personnel_check,
                Icon { icon: Icons::UserCheck, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-sync") }
                }
            }
            button {
                class: "hover:btn-accent join-item",
                onclick: move |_| {
                    navigator().push(route!(API_PERSONNEL, "add"));
                },
                Icon { icon: Icons::Plus, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-add") }
                }
            }
            button {
                class: "hover:btn-neutral join-item",
                onclick: from_clipboard,
                Icon { icon: Icons::Paste, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-clipboard-paste") }
                }
            }
            button {
                class: "hover:btn-neutral join-item",
                onclick: to_clipboard,
                Icon { icon: Icons::Copy, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-clipboard-copy") }
                }
            }
            input {
                class: "hidden",
                id: personnel_import_element_id,
                r#type: "file",
                accept: ".json",
                multiple: false,
                onchange: from_file
            }
            button {
                class: "hover:btn-neutral join-item",
                onclick: |event| {
                    event.prevent_default();
                    event.stop_propagation();
                    jsFfiClickElement(personnel_import_element_id);
                },
                Icon { icon: Icons::Upload, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-upload") }
                }
            }
            button {
                class: "hover:btn-neutral join-item",
                onclick: to_file,
                Icon { icon: Icons::Download, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-export") }
                }
            }
            button {
                class: "hover:btn-error join-item",
                onclick: move |_| users.write().clear(),
                Icon { icon: Icons::Close, class: "size-6" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-clear") }
                }
            }
        }
    }
}
