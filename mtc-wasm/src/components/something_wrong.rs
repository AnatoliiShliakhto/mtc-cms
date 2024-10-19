use super::*;

#[component]
pub fn SomethingWrong<T: PartialEq + 'static>(
    #[props]
    future: Option<Resource<T>>
) -> Element {

    rsx! {
        section {
            class: "div-centered",
            div {
                class: "flex mx-5 flex-col self-center m-fit gap-5 items-center",
                span {
                    class: "flex justify-center text-9xl text-neutral",
                    "500"
                }
                span {
                    class: "text-4xl text-neutral",
                    { t!("message-something-wrong") }
                }
                div {
                    class: "inline-flex divide-x divide-neutral",
                    button {
                        class: "link link-hover hover:text-primary pr-3",
                        onclick: move |_| { navigator().push(route!()); },
                        { t!("action-home") }
                    }
                    if future.is_some() {
                        button {
                            class: "link link-hover hover:text-primary px-3",
                            onclick: move |_| future.unwrap().restart(),
                            { t!("action-try-again") }
                        }
                    }
                    button {
                        class: "link link-hover hover:text-primary pl-3",
                        onclick: move |_| { navigator().push(route!(API_SIGN_IN)); },
                        { t!("action-sign-in") }
                    }
                }
            }
        }
    }
}