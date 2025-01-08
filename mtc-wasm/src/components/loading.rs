use super::*;

/// A component that renders a loading animation.
///
/// This component displays a centered div containing an inline-flex layout
/// with a loading spinner and a loading text. The spinner is styled using
/// the "loading-bars" and "loading-lg" classes. The text displayed alongside
/// the spinner is localized using the `t!` macro with the "action-loading" key.
#[component]
pub fn Loading() -> Element {

    rsx! {
        div { 
            class: "div-centered",
            div { 
                class: "inline-flex items-center gap-3",
                span { 
                    class: "loading loading-bars loading-lg" 
                }
                span { 
                    { t!("action-loading") } 
                }
            }
        }    
    }
}