use super::*;

pub fn Menu() -> Element {
    let auth_state = &*use_auth_state().read_unchecked();

    rsx! {
        MenuItem {
            route: route!(),
            Icon { icon: Icons::Home, class: "size-8 sm:size-6" }
            { t!("menu-home") }
        }
    }
}