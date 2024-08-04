use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::APP_STATE;
use crate::page::administrator::migration::Migration;

#[component]
pub fn HomePage() -> Element {
    let app_state = APP_STATE.peek();
    let i18 = use_i18();

    let mut breadcrumbs = app_state.breadcrumbs.signal();
    use_effect(move || {
        breadcrumbs.set(vec![]);
    });

    let mut migrate = use_signal(|| false);

    rsx! {
        div { class: crate::DIV_CENTER,
            if migrate() {
                div { class: "flex w-full flex-col rounded border p-5 shadow-md max-w-96 input-bordered",
                    Migration {}
                }                
            } else {
                div { class: "flex flex-col flex flex-col w-full items-center gap-5",
                    p { class: "text-4xl",
                        "HOME PAGE / ГОЛОВНА СТОРІНКА"
                    }
                    div { class: "flex gap-5 flex-inline",
                        button { class: "btn btn-primary",
                            onclick: move |_| migrate.set(true),
                            { translate!(i18, "messages.migration") }
                        }
                    }
                }   
            }    
        }
    }
}
