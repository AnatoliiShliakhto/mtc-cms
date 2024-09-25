use super::*;

pub fn Administrator() -> Element {
    build_breadcrumbs("menu-administrator");

    let auth_state = use_auth_state();
    if !auth_state().is_admin() {
        return rsx! { AccessForbidden {} }
    }

    rsx! {}
}