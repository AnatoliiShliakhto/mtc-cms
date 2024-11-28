use std::ops::DerefMut;
use super::*;

#[component]
pub fn AppData() -> Element {
    breadcrumbs!("menu-app-data");

    let is_web = state!(platform).eq("web");

    let mut files = use_signal(BTreeMap::<String, i32>::new);
    let files_size = use_memo(move || {
        let mut count = 0;
        files().iter().for_each(|(_, size)| count += size);
        human_bytes(count)
    });

    if !is_web {
        use_hook(move || {
            spawn( async move {
                let Ok(result) = eval(JS_DOWNLOADS).recv::<Vec<FileEntry>>().await
                else { return };
                files.set(result.iter().map(|file| (
                    file.path.to_string(),
                    file.size
                )).collect::<BTreeMap<String, i32>>());
            });
        })
    }

    let clear_cache = Callback::new(move |event: MouseEvent| {
        spawn(async move {
            if let Ok(Value::Bool(result)) = eval(JS_SW_CLEAR_CACHE).recv().await {
                success_dialog!("message-success-cache-clear")
            } else {
                error_dialog!("error-cache-clear")
            }
        });
    });

    let clear_downloads = Callback::new(move |event: MouseEvent| {
        spawn(async move {
            if let Ok(Value::Bool(result)) = eval(JS_CLEAR_DOWNLOADS).recv().await {
                files.write_unchecked().deref_mut().clear();
                success_dialog!("message-success-downloads-clear")
            } else {
                error_dialog!("error-downloads-clear")
            }
        });
    });

    let delete_file = move |filename: String| {
        let js_eval = JS_REMOVE_DOWNLOAD.replace("{file}", &filename);
        spawn(async move {
            if eval(&js_eval).await.is_ok() {
                files.write_unchecked().deref_mut().remove(&filename);
            }
        });
    };

    rsx! {
        section {
            class: "flex grow select-none flex-col gap-6 px-3",
            h3 {
                class: "flex w-full flex-wrap pb-4 sm:px-4 justify-center text-2xl font-semibold",
                { t!("caption-application-data") }
            }
            div {
                class: "flex grow flex-wrap gap-5",

                div {
                    class: "stats w-80 shadow-md",
                    div {
                        class: "stat",
                        div {
                            class: "stat-figure text-error",
                            Icon { icon: Icons::Database, class: "size-12" }
                        }
                        div {
                            class: "stat-title",
                            { t!("message-app-cache-title") }
                        }
                        div {
                            class: "stat-value",
                            "∞"
                        }
                        div {
                            class: "stat-desc",
                            { t!("message-app-cache-description") }
                        }
                        div {
                            class: "stat-actions",
                            button {
                                class: "btn btn-sm",
                                onclick: move |_| {
                                    alert_dialog!("message-confirm-cache-clear", clear_cache);
                                },
                                { t!("action-clear") }
                            }
                        }
                    }
                }

                if !is_web {
                div {
                        class: "stats w-80 shadow-md",
                        div {
                            class: "stat",
                            div {
                                class: "stat-figure text-accent",
                                Icon { icon: Icons::Download, class: "size-12" }
                            }
                            div {
                                class: "stat-title",
                                { t!("message-app-downloads-title") }
                            }
                            div {
                                class: "stat-value",
                                { files_size() }
                            }
                            div {
                                class: "stat-desc",
                                { t!("message-app-downloads-description") }
                            }
                            div {
                                class: "stat-actions",
                                button {
                                    class: "btn btn-sm",
                                    onclick: move |_| {
                                        alert_dialog!(
                                            "message-confirm-downloads-clear",
                                            clear_downloads
                                        );
                                    },
                                    { t!("action-delete-downloads") }
                                }
                            }
                        }
                    }
                }
            }

            if !is_web {
                table {
                    class: "entry-table",
                    thead {
                        class: "sticky bg-base-200",
                        tr {
                            th { { t!("field-file") } }
                            th {
                                class: "w-28",
                                { t!("field-size") }
                            }
                            th { class: "w-12" }
                        }
                    }
                    tbody {
                        for item in files() {{
                            let file_name = item.0.clone();
                            let file_size = item.1;
                            rsx! {
                                tr {
                                    td {
                                        "onclick": format!(
                                            "openIfExists(window.downloadDirectory + '{}')",
                                            file_name,
                                        ),
                                        { file_name.clone() } }
                                    td { { human_bytes(file_size as f64) } }
                                    td {
                                        class: "p-0",
                                        button {
                                            class: "btn btn-xs btn-ghost text-error",
                                            onclick: move |_| delete_file(file_name.clone()),
                                            Icon {
                                                icon: Icons::Close,
                                                class: "size-4"
                                            }
                                        }
                                    }
                                }
                            }
                        }}
                    }
                }

            }
        }
    }
}