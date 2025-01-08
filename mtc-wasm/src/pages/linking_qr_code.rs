use super::*;

/// Displays a QR code for device linking in the Platform application.
///
/// This component checks if the user is authenticated and shows a QR code
/// for linking a device to the user's account. If the user is not authenticated,
/// it redirects to a NotFound page.
///
/// The page includes a header and announcement message, instructing users
/// to scan the QR code in the Platform application. The QR code is loaded
/// asynchronously and displayed with a loading indicator.
///
/// # UI Elements
///
/// - A centered div containing the main hero section.
/// - A header with bold text and an announcement message.
/// - A card containing the QR code image, loaded with specific attributes
///   for performance and cross-origin security.
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