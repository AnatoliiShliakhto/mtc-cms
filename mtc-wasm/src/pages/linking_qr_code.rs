use super::*;

#[component]
pub fn LinkingQrCode() -> Element {
    if !state!(auth).is_authenticated() {
        return rsx! { NotFound {} }
    }

    breadcrumbs!("menu-linking-qr-code");

    rsx! {
        div {
            class: "div-centered",
            div {
                class: "hero",
                div {
                    class: "hero-content flex-col lg:flex-row-reverse",
                    div {
                        class: "text-center lg:text-left w-full sm:max-w-sm",
                        h1 {
                            class: "text-3xl font-bold",
                            { t!("message-linking-qr-header") }
                        }
                        p {
                            class: "py-6",
                            { t!("message-linking-qr-announcement") }
                        }
                    }
                    div {
                        class: "card w-full max-w-sm shrink-0 border input-bordered rounded",
                        div {
                            class: "card-body",
                            img {
                                src: url!(API_AUTH, "qr"),
                                crossorigin: "use-credentials",
                                decoding: "async",
                                loading: "lazy",
                                alt: "QR-Code"
                            }
                        }
                    }
                }
            }
        }
    }
}