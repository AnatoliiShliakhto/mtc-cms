use super::*;

#[component]
pub fn Breadcrumbs() -> Element {
    let breadcrumbs = state!(breadcrumbs);

    if breadcrumbs.is_empty() {
        return rsx! {}
    }

    rsx! {
        div {
            class: "bg-base-100 text-base-content sm:sticky top-12 z-[20] flex \
                    w-full px-5 bg-opacity-90 backdrop-blur transition-shadow \
                    duration-100 [transform:translate3d(0,0,0)] qr-element",
            div {
                class: "breadcrumbs",
                ul {
                    li {
                        a { class: "gap-2",
                            onclick: move |_| { navigator().push(route!()); },
                            Icon { icon: Icons::Home, class: "size-4" }
                            { t!("menu-home") }
                        }
                    }
                    for item in breadcrumbs.into_iter() {
                        if item.1.is_empty() {
                            li {
                                span {
                                    class: "inline-flex items-center gap-2",
                                    { item.0 }
                                }
                            }
                        } else {
                            li {
                                a {
                                    onclick: move |_| { navigator().push(&*item.1); },
                                    { item.0 }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}