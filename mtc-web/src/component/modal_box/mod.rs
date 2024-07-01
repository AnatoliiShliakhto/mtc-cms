use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::APP_STATE;
use crate::model::modal_model::ModalModel;

pub fn ModalBoxComponent() -> Element {
    let mut modal_model = APP_STATE.peek().modal.signal();
    if modal_model() == ModalModel::None { return rsx! {}; }

    let i18 = use_i18();

    let dialog_close = move |_| {
        modal_model.set(ModalModel::None)
    };

    rsx! {
        input {
            class: "modal-toggle",
            r#type: "checkbox",
            id: "modal_box",
            checked: true,
        }
        div {
            class: "modal",
            role: "dialog",
            "open": "true",
            div { class: "modal-box",
                button {
                    class: "absolute top-2 right-2 btn btn-sm btn-circle btn-ghost",
                    prevent_default: "onclick",
                    onclick: dialog_close,
                    "✕"
                }
                match modal_model() {
                    ModalModel::None => rsx! {},
                    ModalModel::Alert(message) => rsx! {
                        div {
                            class: "flex flex-row gap-4 text-lg font-bold wrap",
                            svg {
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "viewBox": "0 0 24 24",
                                class: "stroke-info shrink-0 w-10 h-10",
                                path {
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    "d": "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                                    "stroke-linecap": "round"
                                }
                            }
                            div { class: "flex grow flex-col",
                                { translate!(i18, "messages.caption_alert") }
                                div { class: "my-0 divider" }
                            }
                        }
                        p { class: "indent-14", { message } }                         
                    },
                    ModalModel::Info(message) => rsx! {
                        div {
                            class: "flex flex-row gap-4 text-lg font-bold wrap",
                            svg {
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "viewBox": "0 0 24 24",
                                class: "stroke-info shrink-0 w-10 h-10",
                                path {
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    "d": "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                                    "stroke-linecap": "round"
                                }
                            }
                            div { class: "flex grow flex-col",
                                { translate!(i18, "messages.caption_info") }
                                div { class: "my-0 divider" }
                            }
                        }
                        p { class: "indent-14", { message } }                             
                    },
                    ModalModel::Error(message) => rsx! {
                        div {
                            class: "flex flex-row gap-4 text-lg font-bold text-error wrap",
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                "fill": "none",
                                "viewBox": "0 0 24 24",
                                class: "h-10 w-10 shrink-0 stroke-current",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-width": "2",
                                    "d": "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                                    "stroke-linejoin": "round"
                                }
                            }
                            div { class: "flex grow flex-col",
                                { translate!(i18, "messages.caption_error") }
                                div { class: "my-0 divider" }
                            }
                        }
                        p { class: "indent-14", { message } } 
                    },
                    ModalModel::Success(message) => rsx! {
                        div {
                            class: "flex flex-row gap-4 text-lg font-bold text-success wrap",
                            svg {
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "viewBox": "0 0 24 24",
                                class: "h-10 w-10 shrink-0 stroke-current",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    "d": "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                }
                            }
                            div { class: "flex grow flex-col",
                                { translate!(i18, "messages.caption_success") }
                                div { class: "my-0 divider" }
                            }
                        }
                        p { class: "indent-14", { message } }                          
                    },
                    ModalModel::Warning(message) => rsx! {
                        div {
                            class: "flex flex-row gap-4 text-lg font-bold text-warning wrap",
                            svg {
                                "viewBox": "0 0 24 24",
                                "fill": "none",
                                "xmlns": "http://www.w3.org/2000/svg",
                                class: "h-10 w-10 shrink-0 stroke-current",
                                path {
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    "stroke-linecap": "round",
                                    "d": "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                                }
                            }
                            div { class: "flex grow flex-col",
                                { translate!(i18, "messages.caption_warning") }
                                div { class: "my-0 divider" }
                            }
                        }
                        p { class: "indent-14", { message } }                         
                    },
                }
                div {
                    class: "modal-action",
                    label {
                        class: "btn btn-outline",
                        prevent_default: "onclick",
                        onclick: dialog_close,
                        { translate!(i18, "messages.close") }
                    }
                }
            }
        }        
    }
}