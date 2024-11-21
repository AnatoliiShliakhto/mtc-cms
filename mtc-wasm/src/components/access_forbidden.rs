use super::*;

#[component]
pub fn AccessForbidden() -> Element {

    rsx! {
        section {
            class: "div-centered",
            div {
                class: "flex mx-5 flex-col self-center m-fit gap-5 items-center",
                span {
                    class: "flex justify-center text-9xl text-neutral",
                    "403"
                }
                span {
                    class: "text-4xl text-neutral",
                    { t!("message-access-forbidden") }
                }
                div {
                    class: "inline-flex divide-x divide-neutral",
                    button {
                        class: "link link-hover hover:text-primary pr-3",
                        onclick: move |_| { navigator().push(route!()); },
                        { t!("action-home") }
                    }
                    if navigator().can_go_back() {
                        button {
                            class: "link link-hover hover:text-primary px-3",
                            onclick: move |_| navigator().go_back(),
                            { t!("action-back") }
                        }
                    }
                    button {
                        class: "link link-hover hover:text-primary pl-3",
                        onclick: move |_| { navigator().push(route!(API_AUTH, API_SIGN_IN)); },
                        { t!("action-sign-in") }
                    }
                }
            }
        }
    }
}