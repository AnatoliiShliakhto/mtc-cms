use super::*;

/// A component to display the content of the private or public storage.
///
/// The component has the following properties:
///
/// * `id`: The id of the storage to show.
/// * `is_private`: A boolean indicating whether the storage is private or not.
/// * `is_show`: A signal indicating whether the component should be shown or not.
///
/// The component will render a modal window containing the content of the
/// storage. The window will have a title indicating the type of storage, a
/// refresh button and a close button. The content of the storage will be
/// rendered as a table with three columns: file name, file size and a delete
/// button. The delete button will only be visible if the user has the
/// permission to delete files.
///
/// When the user clicks on the file name, the file will be copied to the
/// clipboard. When the user clicks on the delete button, the file will be
/// deleted. When the user clicks on the refresh button, the content of the
/// storage will be refreshed. When the user clicks on the close button, the
/// component will be hidden.
///
/// The component will also render a progress bar indicating the progress of
/// the file upload. The progress bar will be red if an error occurred, green
/// if the file was uploaded successfully and blue if the file is being
/// uploaded.
#[component]
pub fn StorageBox(
    #[props(into)]
    id: String,
    #[props(into)]
    is_private: bool,
    #[props]
    is_show: Signal<bool>,
) -> Element {
    if !is_show() { return rsx! {} }

    let auth = state!(auth);
    let mut download_progress = use_signal(|| 0);

    let id = use_memo(use_reactive!(|id| id));
    let is_private = use_memo(use_reactive!(|is_private| is_private));

    let api = use_memo(move || {
        let api = if is_private() {
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
        auth.has_permission(PERMISSION_PRIVATE_STORAGE_WRITE)
    } else {
        auth.has_permission(PERMISSION_PUBLIC_STORAGE_WRITE)
    };
    let can_delete = if is_private() {
        auth.has_permission(PERMISSION_PRIVATE_STORAGE_DELETE)
    } else {
        auth.has_permission(PERMISSION_PUBLIC_STORAGE_DELETE)
    };

    let mut future = value_future!(api());

    let delete_file = move |filename: String| {
        let url = format!("{}/{}", api(), filename.clone());
        spawn(async move {
            if delete_request!(url) {
                future.restart()
            }
        });
    };

    let upload_task = move |event: Event<FormData>| {
        spawn(async move {
            loop {
                match eval(
                    if is_private() {
                        JS_PRIVATE_FILE_UPLOAD
                    } else {
                        JS_PUBLIC_FILE_UPLOAD
                    }
                ).recv().await {
                    Ok(Value::Number(value)) => {
                        download_progress.set(value.as_i64().unwrap_or_default())
                    }
                    Ok(Value::String(value)) => {
                        if value.is_empty() {
                            download_progress.set(100);
                            if let Some(file_engine) = &event.files() {
                                let files = file_engine.files();
                                if files.len() == 1 {
                                    match eval(JS_COPY_TO_CLIPBOARD)
                                        .send(format!("{:0}/{:1}", path(), files[0])) {
                                        Ok(_) => {
                                            download_progress.set(0);
                                            eval(JS_FILE_INPUTS_CLEAR);
                                            is_show.set(false)
                                        },
                                        Err(_) => {}
                                    }
                                } else {
                                    future.restart()
                                }
                            }
                        } else {
                            download_progress.set(101)
                        }
                        break
                    }
                    _ => {
                        download_progress.set(101);
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
                            class: "file-input mt-1 w-full",
                            class: if download_progress().eq(&101i64) {
                                "file-input-error"
                            } else if download_progress().eq(&100i64) {
                                "file-input-success"
                            } else {
                                "file-input-info"
                            },
                            r#type: "file",
                            id: "fileUpload",
                            multiple: false,
                            disabled: !can_write,
                            onchange: upload_task,
                        }
                    }
                    progress {
                        class: "progress w-full mt-3 mb-1",
                        class: if download_progress().eq(&101i64) {
                            "progress-error"
                        } else if download_progress().eq(&100i64) {
                            "progress-success"
                        } else {
                            "progress-info"
                        },
                        value: download_progress(),
                        max: 100,
                    }
                }
                div {
                    class: "flex items-start overflow-auto",
                    style: "height: calc(100% - 8rem)",
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
                                tbody {
                                    for item in response.self_obj::<Vec<FileAsset>>()
                                    .unwrap_or_default().iter() {{
                                        let file_name = item.name.clone();
                                        let file_path = format!("{}/{}", path(), item.name);
                                        let file_size = item.size;
                                        rsx! {
                                            tr {
                                                onclick: move |_| {
                                                    match eval(JS_COPY_TO_CLIPBOARD)
                                                    .send(file_path.clone()) {
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
                            }
                        },
                        None => rsx! { Loading {} },
                    }
                }
            }
        }
    }
}
