use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use serde_json::Value;

use mtc_model::user_details_model::UserDetailsModel;

use crate::model::modal_model::ModalModel;
use crate::service::user_service::UserService;
use crate::APP_STATE;

#[component]
pub fn Dashboard() -> Element {
    let i18 = use_i18();
    /*
        let mut clipboard_eval = eval(r#"
            let msg = await dioxus.recv();
            console.log(msg);
             const link = document.createElement("a");
             const file = new Blob([msg], { type: "text/plain;charset=utf-8" });
             link.href = URL.createObjectURL(file);
             link.download = "sample.json";
             link.click();
             URL.revokeObjectURL(link.href);
            "#);
    */

    let clipboard_write_eval = eval(
        r#"
        let msg = await dioxus.recv();
        navigator.clipboard.write([new ClipboardItem({'text/plain': new Blob([msg], {type: 'text/plain;charset=utf-8'})})]);
        "#,
    );

    let clipboard_read_eval = eval(
        r#"
        navigator.clipboard.readText().then((clipText) => (dioxus.send(clipText)));
        "#,
    );

    let users_from_clipboard = move |_| {
        spawn(async move {
            to_owned![clipboard_read_eval];
            match clipboard_read_eval.recv().await {
                Ok(Value::String(value)) => APP_STATE.peek().users.signal().set(BTreeMap::<
                    String,
                    UserDetailsModel,
                >::from_string(
                    &value
                )),
                _ => APP_STATE
                    .peek()
                    .modal
                    .signal()
                    .set(ModalModel::Error(translate!(i18, "errors.clipboard"))),
            }
        });
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
        div { class: "flex grow flex-row",
            div { class: "flex grow flex-col items-center gap-3 p-5 body-scroll",
                table { class: "table w-full",
                    thead {
                        tr {
                            th { style: "width: 1.75rem;" }
                            th { { translate!(i18, "messages.login") } }
                            th { { translate!(i18, "messages.rank") } }
                            th { { translate!(i18, "messages.name") } }
                        }
                    }
                    tbody {
                        for item in users(){
                            tr {
                                td {
                                    button { class: "btn btn-sm btn-ghost",
                                        prevent_default: "onclick",
                                        onclick: move |_| remove_user(item.0.clone()),
                                        "❌"
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
            div { class: "flex flex-col gap-3 p-5 min-w-36 body-scroll",
                button {
                    class: "btn btn-accent btn-outline",
                    prevent_default: "onclick",
                    onclick: users_from_clipboard,
                    Icon {
                            width: 16,
                            height: 16,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::fa_regular_icons::FaPaste
                    }
                    { translate!(i18, "messages.clipboard_paste") }
                }
                button {
                    class: "btn btn-warning btn-outline",
                    prevent_default: "onclick",
                    onclick: move |_| { clipboard_write_eval.send(users().get_users_json().into()).unwrap(); },
                    Icon {
                            width: 16,
                            height: 16,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::fa_regular_icons::FaCopy
                    }
                    { translate!(i18, "messages.clipboard_copy") }
                }
            }
        }
    }
}
