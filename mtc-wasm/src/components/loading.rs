use super::*;

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