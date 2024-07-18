use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use futures_util::StreamExt;
use mtc_model::api_model::ApiListItemModel;
use mtc_model::auth_model::AuthModelTrait;

use crate::APP_STATE;
use crate::element::editor::{ContentEdit, Editor};
use crate::handler::content_handler::ContentHandler;
use crate::page::not_found::NotFoundPage;

#[derive(Props, Clone, PartialEq)]
pub struct ContentProps {
    pub schema: Signal<Option<String>>,
}

enum ContentActions {
    Update(String),
}

#[component]
pub fn Content(props: ContentProps) -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();
    let i18 = use_i18();

    let schema = props.schema;

    if !auth_state.is_permission("administrator") {
        return rsx! { NotFoundPage {} };
    }

    let mut content_edit = use_signal(|| Option::<ContentEdit>::None);
    let mut content_list = use_signal(BTreeMap::<usize, ApiListItemModel>::new);

    let content_controller =
        use_coroutine(|mut rx: UnboundedReceiver<ContentActions>| async move {
            while let Some(msg) = rx.next().await {
                match msg {
                    ContentActions::Update(schema) => {
                        if let Ok(value) = APP_STATE.peek().api.get_content_list(&schema).await {
                            let mut list = BTreeMap::<usize, ApiListItemModel>::new();
                            for (count, item) in value.iter().enumerate() {
                                list.insert(count, item.clone());
                            }
                            content_list.set(list);
                        }
                    }
                }
            }
        });

    use_effect(move || {
        if content_edit().is_none() {
            content_controller.send(ContentActions::Update(schema().unwrap_or_default()))
        }
    });
    
/*
    use_effect(move || {
        content_controller.send(ContentActions::Update(schema()))
    });
*/
    use_hook(|| {
        content_edit.set(None);
        content_controller.send(ContentActions::Update(schema().unwrap_or_default()));
    });

    let mut edit_task = move |id: usize| {
        let item = content_list().get_key_value(&id).unwrap().1.clone();
        content_edit.set(Some(ContentEdit {
            schema: schema(),
            api: item.slug,
            is_new: false,
        }));
    };

    if content_edit().is_some() {
        return rsx! { Editor { content: content_edit } };
    }
    
    let add_content = move |_| {
        content_edit.set(Some(ContentEdit {
            schema: schema(),
            api: String::new(),
            is_new: true,
        }));        
    };

    rsx! {
        div { class: "flex grow flex-row",
            div { class: "flex grow flex-col items-center gap-3 p-5 body-scroll",
                table { class: "table w-full",
                    thead {
                        tr {
                            th { class: "w-6" }
                            th { { translate!(i18, "messages.slug") } }
                            th { { translate!(i18, "messages.title") } }
                        }
                    }
                    tbody {
                        for (id, item) in content_list() {
                            tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                onclick: move |event| {
                                    event.stop_propagation();
                                    edit_task(id)
                                },
                                td {
                                    if !item.published {
                                           Icon { class: "text-warning",
                                            width: 16,
                                            height: 16,
                                            fill: "currentColor",
                                            icon: dioxus_free_icons::icons::md_action_icons::MdVisibilityOff
                                        }
                                    }
                                }
                                td { { item.slug } }
                                td { { item.title } }
                            }
                        }
                    }
                }
            }
        }
        if schema().is_some() {
            button {
                class: "absolute right-4 bottom-4 btn btn-circle btn-accent",
                prevent_default: "onclick",
                onclick: add_content,
                Icon {
                    width: 26,
                    height: 26,
                    icon: dioxus_free_icons::icons::md_content_icons::MdAdd
                }
            }
        }
    }
}
