use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use human_bytes::human_bytes;
use serde_json::Value;
use tracing::error;
use mtc_model::auth_model::AuthModelTrait;
use crate::{APP_STATE};
use crate::component::loading_box::LoadingBoxComponent;
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::handler::storage_handler::StorageHandler;
use crate::model::modal_model::ModalModel;

#[derive(Props, Clone, PartialEq)]
pub struct StorageProps {
    pub dir: Memo<String>,
    pub is_shown: Signal<bool>,
    pub private: bool,
}

#[component]
pub fn StorageManager(mut props: StorageProps) -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();
    let storage_dir = props.dir;
    let storage_api = use_memo(move || {
        if props.private {
            format!("{:0}/{:1}", "private_storage", storage_dir())
        } else {
            format!("{:0}/{:1}", "storage", storage_dir())
        }
    });
    let file_path = use_memo(move || {
        if props.private {
            format!("{:0}/{:1}", crate::PRIVATE_STORAGE_URL, storage_dir())
        } else {
            format!("{:0}/{:1}", crate::PUBLIC_STORAGE_URL, storage_dir())
        }
    });
    let write_permission = use_memo(move || {
        let app_state = APP_STATE.peek();
        let auth_state = app_state.auth.read();
        
        if props.private {
            auth_state.is_permission("private_storage::write")
        } else {
            auth_state.is_permission("storage::write")
        }
    });
    let delete_permission = use_memo(move || {
        let app_state = APP_STATE.peek();
        let auth_state = app_state.auth.read();

        if props.private {
            auth_state.is_permission("private_storage::delete")
        } else {
            auth_state.is_permission("storage::delete")
        }
    });

    let mut progress = use_signal(|| 0);

    let mut storage_future =
        use_resource(
            move || async move {
                APP_STATE.peek().api.get_storage_files(&storage_api()).await
            },
        );

    let mut copy_to_clipboard = move |file_path: String| {
        match eval(r#"
            let msg = await dioxus.recv();
            navigator.clipboard.write([new ClipboardItem({'text/plain': new Blob([msg], {type: 'text/plain;charset=utf-8'})})]);
            "#).send(Value::String(file_path))
        {
            Ok(_) => props.is_shown.set(false),
            Err(e) => error!("{:#?}", e),
        }
    };

    let delete_file = move |file_name: String| {
        spawn(async move {
            match APP_STATE
                .peek()
                .api
                .delete_file(&storage_api(), &file_name)
                .await
            {
                Ok(_) => storage_future.restart(),
                Err(e) => APP_STATE
                    .peek()
                    .modal
                    .signal()
                    .set(ModalModel::Error(e.message())),
            }
        });
    };

    //todo multi files upload

    let upload_task = move |_| {
        let file_uploader = eval(
            &[
                r#"
            var formData = new FormData();
            var fileInput = document.getElementById('fileUpload');
            var file = fileInput.files[0];

            formData.append('file', file);

            var xhr = new XMLHttpRequest();

            xhr.upload.addEventListener('progress', function (event) {
                if (event.lengthComputable) {
                    var percent = Math.round((event.loaded / event.total) * 100);
                    dioxus.send(percent);
                }
            });

            xhr.addEventListener('load', function (event) {
                dioxus.send(event.target.responseText);
            });

            xhr.open('POST', '/api/"#,
                storage_api().as_str(),
                r#"', true);
            xhr.send(formData);
        "#,
            ]
                .concat(),
        );

        spawn(async move {
            to_owned![file_uploader];
            loop {
                match file_uploader.recv().await {
                    Ok(Value::Number(value)) => progress.set(value.as_i64().unwrap_or_default()),
                    Ok(Value::String(value)) => {
                        if value.is_empty() {
                            progress.set(100);
                            storage_future.restart()
                        } else {
                            progress.set(101)
                        }
                        break;
                    }
                    _ => {
                        progress.set(101);
                        break;
                    }
                }
            }
        });
    };

    rsx! {
        section { class: "modal modal-open",
            onclick: move |_| props.is_shown.set(false),
            div { class: "modal-box min-w-96 w-fit h-5/6",
                onclick: move |event| event.stop_propagation(),
                div { class: "h-32 overflow-auto",
                    button {
                        class: "absolute top-2 right-2 btn btn-sm btn-circle btn-ghost",
                        onclick: move |_| props.is_shown.set(false),
                        "✕"
                    }
                    if props.private {
                        h1 { class: "text-title text-lg", { translate!(i18, "messages.private_storage") } }
                     } else {
                        h1 { class: "text-title text-lg", { translate!(i18, "messages.public_storage") } }
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
                            disabled: !write_permission(),
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
                div { class: "flex items-start overflow-auto", style: "height: calc(100% - 8rem)",
                    match &*storage_future.read() {
                        Some(Ok(response)) => rsx! {
                            table { class: "table w-full",
                                thead { class: "sticky top-[-1px] bg-base-200",
                                    tr {
                                        th { { translate!(i18, "messages.file") } }
                                        th { { translate!(i18, "messages.size") } }
                                        if delete_permission() {
                                            th { class: "w-6" }
                                        }    
                                    }
                                }

                                for item in response.files.iter() {
                                    {
                                        let file_name = item.name.clone();
                                        let file_path = format!("{:0}/{:1}", file_path(), item.name);
                                        let file_size = item.size;
                                        rsx! {
                                            tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                                onclick: move |_| { copy_to_clipboard(file_path.clone()) },
                                                td { { file_name.clone() } }
                                                td { { human_bytes(file_size as f64) } }
                                                if delete_permission() {
                                                    td {
                                                        onclick: move |event| event.stop_propagation(),
                                                        button { class: "btn btn-xs btn-ghost text-error",
                                                            onclick: move |_| delete_file(file_name.clone()),
                                                            Icon {
                                                                width: 18,
                                                                height: 18,
                                                                fill: "currentColor",
                                                                icon: dioxus_free_icons::icons::md_navigation_icons::MdClose
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! { 
                            div { class: "grid w-full h-full place-items-center",
                                ReloadingBoxComponent { message: e.message(), resource: storage_future }
                            }    
                        },
                        None => rsx! {
                            div { class: "grid w-full h-full place-items-center",
                                LoadingBoxComponent {}
                            }    
                        },
                    }
                }
            }
        }
    }
}
