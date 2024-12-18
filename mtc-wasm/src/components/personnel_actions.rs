use super::*;

#[component]
pub fn PersonnelActions() -> Element {
    let mut users = state_fn!(personnel);
    let columns = state_fn!(personnel_columns);
    let column_login = columns.login;
    let column_rank = columns.rank;
    let column_name = columns.name;
    let column_password = columns.password;
    let column_group = columns.group;
    let column_access = columns.access;

    let from_clipboard = move |_| async move {
        if let Ok(Value::String(value)) = eval(JS_PASTE_FROM_CLIPBOARD).recv().await {
            let mut reader = csv::ReaderBuilder::new()
                .delimiter(b'\t')
                .has_headers(false)
                .flexible(true)
                .trim(csv::Trim::All)
                .from_reader(value.as_bytes());

            users.write().clear();
            for item in reader.deserialize::<PersonDto>() {
                if let Ok(item) = item {
                    let login = item.login.replace(" ", "").to_uppercase();
                    users.write().insert(login.clone().into(), UserDetails {
                        login: login.into(),
                        rank: item.rank.trim().to_string().to_lowercase().into(),
                        name: item.name.trim().to_string().into(),
                        ..Default::default()
                    });
                }
            }
        }
    };

    let to_clipboard = move |_| async move {
        let payload = Value::String(std::iter::Map::collect::<Vec<String>>(
            users().iter()
                .map(|(login, details)| {
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
                                    access.format("%d/%m/%Y").to_string(),
                                    &details.access_count
                                ).into()
                            )
                        } else { array.push("".into()); }
                    }

                    array.join("\t")
                }),
        )
            .join("\r\n"));

        if eval(JS_COPY_TO_CLIPBOARD).send(payload).is_ok() {
            success_dialog!("message-personnel-copy-successful");
        }
    };

    let from_file = move |event: Event<FormData>| async move {
        if let Some(file_engine) = event.files() {
            let files = file_engine.files();
            if files.is_empty() { return }
            if let Some(json_string) = file_engine.read_file_to_string(&files[0]).await {
                let Ok(personnel) =
                    serde_json::from_str::<Vec<PersonDto>>(&json_string) else { return };

                users.write().clear();
                personnel.iter().for_each(|item| {
                    users.write().insert(item.login.clone(), UserDetails {
                        login: item.login.clone(),
                        rank: item.rank.clone(),
                        name: item.name.clone(),
                        ..Default::default()
                    });
                });
            }
            let _ = eval(JS_FILE_INPUTS_CLEAR).await.ok();
        }
    };

    let to_file = move |_| async move {
        let payload = users()
            .iter()
            .map(|(login, details)| PersonDto {
            login: details.login.clone(),
            rank: details.rank.clone(),
            name: details.name.clone(),
        }).collect::<Vec<PersonDto>>();

        if eval(JS_EXPORT_PERSONNEL).send(payload).is_ok() {
            success_dialog!("message-personnel-export-successful");
        }
    };

    let personnel_check = move |_| {
        let payload = json!(users()
            .iter()
            .map(|(login, _)| login.clone())
            .collect::<Vec<Cow<'static, str>>>());

        spawn(async move {
            let response = value_request!(url!(API_PERSONNEL), payload);
            let Some(user_details_dto) =
                response.self_obj::<Vec<UserDetailsDto>>() else { return };
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
                id: "personnel-upload",
                r#type: "file",
                accept: ".json",
                multiple: false,
                onchange: from_file
            }
            button {
                class: "hover:btn-neutral join-item",
                "onclick": "document.getElementById('personnel-upload').click()",
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