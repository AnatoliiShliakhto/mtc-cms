use super::*;

/// A basic menu component that provides a link to the home page.
pub fn Menu() -> Element {
    let auth = state!(auth);

    rsx! {
        MenuItem {
            route: route!(),
            Icon { icon: Icons::Home, class: "size-8 sm:size-6" }
            { t!("menu-home") }
        }
    }
}