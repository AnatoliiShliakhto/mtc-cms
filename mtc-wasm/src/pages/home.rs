use super::*;

pub fn Home() -> Element {
    drop_breadcrumbs();

    rsx!{ 
        div { 
            class: "div-centered",
            div { 
                class: "flex grow flex-col gap-4 p-5 items-center",
                img { 
                    class: "size-40", 
                    src: "/assets/logo.png" 
                }
                span { 
                    class: "text-2xl mt-6 font-semibold text-wrap", 
                    { t!("site-home-title") } 
                }
                span { 
                    class: "text-xl font-semibold text-wrap", 
                    { t!("site-home-description") } 
                }
            }
        }
    }
}