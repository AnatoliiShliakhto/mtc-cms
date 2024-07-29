use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use serde_json::Value;
use tracing::error;

use mtc_model::user_details_model::UserDetailsModel;

use crate::APP_STATE;
use crate::component::breadcrumb::Breadcrumb;
use crate::model::modal_model::ModalModel;
use crate::service::user_service::UserService;

#[component]
pub fn Personas() -> Element {
    let i18 = use_i18();

    let users_to_clipboard = move |_| {
        let users = APP_STATE.peek().users.signal();

        match eval(r#"
            let msg = await dioxus.recv();
            navigator.clipboard.write([new ClipboardItem({'text/plain': new Blob([msg], {type: 'text/plain;charset=utf-8'})})]);
        "#).send(users().get_users_string()) {
            Ok(_) => {}
            Err(e) => error!("{:#?}", e),
        }
    };

    let users_from_clipboard = move |_| {
        let clipboard_read_eval = eval(
            r#"
            navigator.clipboard.readText().then((clipText) => (dioxus.send(clipText)));
        "#,
        );

        spawn(async move {
            to_owned![clipboard_read_eval];
            match clipboard_read_eval.recv().await {
                Ok(Value::String(value)) => {
                    let users = APP_STATE.peek().users.signal();
                    APP_STATE
                        .peek()
                        .users
                        .signal()
                        .set(users().import_str(value.as_str()))
                }
                _ => APP_STATE
                    .peek()
                    .modal
                    .signal()
                    .set(ModalModel::Error(translate!(i18, "errors.clipboard"))),
            }
        });
    };

    let download_user_set = move |_| {
        let users_download_eval = eval(
            r#"
            const toObject = (map = new Map()) =>
                Object.fromEntries(Array.from(map.entries(), ([ k, v ]) =>
                    v instanceof Map ? [ k, toObject (v) ] : [ k, v ]
                )
            )
            const obj = JSON.stringify(toObject(await dioxus.recv()));
            const file = new Blob([obj], { type: "application/json" });

            if( window.showSaveFilePicker ) {
                let opts = {
                    types: [{
                    description: 'JSON',
                    accept: {'application/json': ['.json']},
                    }],
                    suggestedName: 'mtc-users',
                };
                var handle = await showSaveFilePicker(opts);
                var writable = await handle.createWritable();
                await writable.write(file);
                writable.close();
            } else { alert( "File save error" ); }
        "#,
        );

        let users = APP_STATE.peek().users.signal();
        if users().is_empty() {
            return;
        }
        users_download_eval.send(users().get_users_json()).unwrap()
    };

    let upload_user_set = move |event: Event<FormData>| async move {
        let users = APP_STATE.peek().users.signal();
        let mut user_details = users.peek().clone();

        if let Some(file_engine) = event.files() {
            let files = file_engine.files();
            for file_name in &files {
                if let Some(file) = file_engine.read_file_to_string(file_name).await {
                    user_details.import_json(file.as_str());
                }
            }
            APP_STATE.peek().users.signal().set(user_details)
        }
    };

    let users = APP_STATE.peek().users.signal();
    /*
       let users_submit = move |event: Event<FormData>| {
           if event.value().is_empty() {
               return;
           }
           APP_STATE
               .peek()
               .users
               .signal()
               .set(BTreeMap::<String, UserDetailsModel>::from_string(
                   &event.value(),
               ));
       };

    */

    let remove_user = move |user: String| {
        APP_STATE
            .peek()
            .users
            .signal()
            .set(users().remove_user(&user));
    };

    rsx! {
        section { class: "flex grow flex-row",
            div { class: "flex grow flex-col items-center gap-3 p-2 body-scroll",
                div { class: "self-start",
                    Breadcrumb { title: translate!(i18, "messages.personas") }
                }
                table { class: "table w-full",
                    thead {
                        tr {
                            th { class: "w-6" }
                            th { { translate!(i18, "messages.login") } }
                            th { { translate!(i18, "messages.rank") } }
                            th { { translate!(i18, "messages.name") } }
                        }
                    }
                    tbody {
                        for item in users(){
                            tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                td {
                                    button { class: "btn btn-xs btn-ghost text-error",
                                        onclick: move |_| remove_user(item.0.clone()),
                                        Icon {
                                            width: 16,
                                            height: 16,
                                            fill: "currentColor",
                                            icon: dioxus_free_icons::icons::md_navigation_icons::MdClose
                                        }
                                    }
                                }
                                td { { item.0.clone() } }
                                td { { item.1.rank.clone() } }
                                td { { item.1.name.clone() } }
                            }
                        }
                    }
                }
            }
            aside { class: "flex flex-col gap-3 p-2 pt-3 shadow-lg bg-base-200 min-w-48 body-scroll",
                h2 { class: "menu-title self-center", { translate!(i18, "messages.external_data") } }
                div { class: "px-2 join",
                    div { class: "tooltip", "data-tip": translate!(i18, "messages.clipboard_paste"),
                        button {
                            class: "join-item btn btn-sm btn-ghost text-accent",
                            onclick: users_from_clipboard,
                            Icon {
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::fa_regular_icons::FaPaste
                            }
                        }
                    }
                    div { class: "tooltip", "data-tip": translate!(i18, "messages.clipboard_copy"),
                        button {
                            class: "join-item btn btn-sm btn-ghost",
                            onclick: users_to_clipboard,
                            Icon {
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::fa_regular_icons::FaCopy
                            }
                        }
                    }
                    input { class: "hidden",
                        id: "users-upload",
                        r#type: "file",
                        accept: ".json",
                        multiple: true,
                        onchange: upload_user_set
                    }
                    div { class: "tooltip", "data-tip": translate!(i18, "messages.upload"),
                        button {
                            class: "join-item btn btn-sm btn-ghost text-accent",
                            "onclick": "document.getElementById('users-upload').click()",
                            Icon {
                                width: 20,
                                height: 20,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::md_file_icons::MdFileUpload
                            }
                        }
                    }
                    div { class: "tooltip", "data-tip": translate!(i18, "messages.download"),
                        button {
                            class: "join-item btn btn-sm btn-ghost",
                            onclick: download_user_set,
                            Icon {
                                width: 20,
                                height: 20,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::md_file_icons::MdFileDownload
                            }
                        }
                    }
                    div { class: "tooltip", "data-tip": translate!(i18, "messages.clear"),
                        button {
                            class: "join-item btn btn-sm btn-ghost text-error",
                            onclick: move |_| APP_STATE.peek().users.signal().set(BTreeMap::<String, UserDetailsModel>::new()),
                            Icon {
                                width: 18,
                                height: 18,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::md_content_icons::MdClear
                            }
                        }
                    }
                }
            }
        }
    }
}
