use super::*;

#[component]
pub fn StorageBox(
    #[props(into)]
    id: String,
    #[props(into)]
    is_private: bool,
    #[props]
    is_show: Signal<bool>,
) -> Element {
    if !is_show() { return rsx!{} }

    let auth_state = use_auth_state();
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let mut progress = use_signal(|| 0);
    let api_client = use_api_client();

    let id = use_memo(use_reactive!(|id| id));
    let is_private = use_memo(use_reactive!(|is_private| is_private));

    let api = use_memo(move || {
        let api: String = if is_private() {
            url!(API_PRIVATE_STORAGE, &id())
        } else {
            url!(API_PUBLIC_STORAGE, &id())
        };
        api
    });

    let path = use_memo(move || {
        if is_private() {
            format!("{}/{}", PRIVATE_ASSETS_PATH, id())
        } else {
            format!("{}/{}", PUBLIC_ASSETS_PATH, id())
        }
    });

    let can_write = if is_private() {
        auth_state().has_permission(PERMISSION_PRIVATE_STORAGE_WRITE)
    } else {
        auth_state().has_permission(PERMISSION_PUBLIC_STORAGE_WRITE)
    };
    let can_delete = if is_private() {
        auth_state().has_permission(PERMISSION_PRIVATE_STORAGE_DELETE)
    } else {
        auth_state().has_permission(PERMISSION_PUBLIC_STORAGE_DELETE)
    };

    let mut future =
        use_resource(
            move || async move {
                request_fetch_task(api().into()).await
            },
        );

    let delete_file = move |filename: String| {
        spawn(async move {
            match api_client()
                .delete(format!("{}/{}", api(), filename.clone()))
                .send()
                .await {
                Ok(_) => future.restart(),
                Err(e) =>
                    message_box_task.send(MessageBoxAction::Error("error-file-delete".into())),
            }
        });
    };

    let upload_task = move |event: Event<FormData>| {
        spawn(async move {
            loop {
                match UseEval::new(document()
                    .new_evaluator(
                        if is_private() {
                            EVAL_PRIVATE_FILE_UPLOAD
                        } else {
                            EVAL_PUBLIC_FILE_UPLOAD
                        }.to_string()
                    )).recv().await {
                    Ok(Value::Number(value)) => {
                        progress.set(value.as_i64().unwrap_or_default())
                    }
                    Ok(Value::String(value)) => {
                        if value.is_empty() {
                            progress.set(100);
                            if let Some(file_engine) = &event.files() {
                                let files = file_engine.files();
                                if files.len() == 1 {
                                    match UseEval::new(document()
                                        .new_evaluator(EVAL_COPY_TO_CLIPBOARD.to_string()))
                                        .send(format!("{:0}/{:1}", path(), files[0]).into()) {
                                        Ok(_) => is_show.set(false),
                                        Err(_) => {}
                                    }
                                } else {
                                    future.restart()
                                }
                            }
                        } else {
                            progress.set(101)
                        }
                        break
                    }
                    _ => {
                        progress.set(101);
                        break;
                    },
                }
            }
        });
    };

    rsx! {
        section {
            class: "modal modal-open",
            onclick: move |_| is_show.set(false),

            div {
                class: "modal-box min-w-96 w-fit h-5/6",
                onclick: move |event| event.stop_propagation(),
                div {
                    if is_private() {
                        Icon {
                            icon: Icons::DatabaseLock,
                            class: "size-6 absolute top-6 left-6 text-warning"
                        }
                    } else {
                        Icon {
                            icon: Icons::Database,
                            class: "size-6 absolute top-6 left-6 text-success"
                        }
                    }
                    div {
                        class: "absolute top-0 right-0 join rounded-none",
                        button {
                            class: "btn btn-sm btn-ghost join-item hover:text-primary",
                            onclick: move |_| future.restart(),
                            Icon { icon: Icons::Refresh, class: "size-4" }
                        }
                        button {
                            class: "btn btn-sm btn-ghost join-item hover:text-error",
                            onclick: move |_| is_show.set(false),
                            Icon { icon: Icons::Close, class: "size-4" }
                        }
                    }
                    h1 {
                        class: "text-title text-lg text-center",
                        if is_private() {
                            { t!("caption-private-storage") }
                        } else {
                            { t!("caption-public-storage") }
                        }
                    }
                    div { class: "divider my-0" }
                    form {
                        input {
                            class: if progress().eq(&101i64) {
                                "file-input file-input-bordered mt-1 file-input-error w-full"
                            } else if progress().eq(&100i64) {
                                "file-input file-input-bordered mt-1 file-input-success w-full"
                            } else {
                                "file-input file-input-bordered mt-1 file-input-info w-full"
                            },
                            r#type: "file",
                            id: "fileUpload",
                            multiple: false,
                            disabled: !can_write,
                            onchange: upload_task,
                        }
                    }
                    progress {
                        class: if progress().eq(&101i64) {
                            "progress w-full mt-3 mb-1 progress-error"
                        } else if progress().eq(&100i64) {
                            "progress w-full mt-3 mb-1 progress-success"
                        } else {
                            "progress w-full mt-3 mb-1 progress-info"
                        },
                        value: progress(),
                        max: 100,
                    }
                }
                div {
                    class: "flex items-start overflow-auto", style: "height: calc(100% - 8rem)",
                    match future() {
                        Some(response) => rsx! {
                            table {
                                class: "entry-table",
                                thead {
                                    class: "sticky top-[-1px] bg-base-200",
                                    tr {
                                        th { { t!("field-file") } }
                                        th {
                                            class: "w-28",
                                            { t!("field-size") }
                                        }
                                        if can_delete {
                                            th { class: "w-12" }
                                        }
                                    }
                                }

                                for item in serde_json::from_value::<Vec<Asset>>(response)
                                .unwrap_or(vec![]).iter() {{
                                    let file_name = item.name.clone();
                                    let file_path = format!("{}/{}", path(), item.name);
                                    let file_size = item.size;
                                    rsx! {
                                        tr {
                                            onclick: move |_| {
                                                match UseEval::new(document()
                                                .new_evaluator(EVAL_COPY_TO_CLIPBOARD.to_string()))
                                                .send(file_path.clone().into()) {
                                                    Ok(_) => is_show.set(false),
                                                    Err(_) => {}
                                                }
                                            },
                                            td { { file_name.clone() } }
                                            td { { human_bytes(file_size as f64) } }
                                            if can_delete {
                                                td {
                                                    class: "p-0",
                                                    onclick: move |event| event.stop_propagation(),
                                                    button {
                                                        class: "btn btn-xs btn-ghost text-error",
                                                        onclick: move |_|
                                                        delete_file(file_name.to_string()),
                                                        Icon {
                                                            icon: Icons::Close,
                                                            class: "size-4"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }}
                            }
                        },
                        None => rsx! { Loading {} },
                    }
                }
            }
        }
    }
}
