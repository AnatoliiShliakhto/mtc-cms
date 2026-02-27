use super::*;

#[component]
pub fn Home() -> Element {
    breadcrumbs!();

    rsx!{ 
        div {
            class: "div-centered",
            div {
                class: "flex grow flex-col gap-4 p-5 items-center",
                img {
                    style: "width: 13rem; height: 13rem;",
                    src: "/assets/logo.webp"
                }
                span {
                    class: "text-4xl mt-6 font-semibold text-wrap",
                    { "242" }
                }
                span {
                    class: "text-2xl font-semibold text-wrap",
                    { "центр підготовки підрозділів" }
                }
            }
        }
    }
}