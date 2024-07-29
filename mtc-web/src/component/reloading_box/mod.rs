use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

#[derive(Props)]
pub struct ReloadingBoxComponentProps<T: 'static>
{
    pub message: String,
    resource: Resource<T>,
}

impl<T: 'static> PartialEq for ReloadingBoxComponentProps<T> {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}

impl<T> Clone for ReloadingBoxComponentProps<T> {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            resource: self.resource,
        }
    }
}

//todo compact
pub fn ReloadingBoxComponent<T: 'static>(
    mut props: ReloadingBoxComponentProps<T>,
) -> Element {
    let i18 = use_i18();

    rsx! {
        div { class: "card bg-base-100 min-w-96 shadow-xl",
            div { class: "card-body",
                p { class: "card-title text-lg text-error", { translate!(i18, "messages.oops") } }
                p { class: "card-title text-lg text-error", { translate!(i18, "messages.something_wrong") } }
                div { class: "divider m-0" }
                p { { props.message } }
                div { class: "card-actions justify-end",
                    button {
                        class: "btn btn-outline btn-accent",
                        onclick: move |_| props.resource.restart(),
                        { translate!(i18, "messages.try_again") }
                    }
                }
            }
        }        
    }
}
