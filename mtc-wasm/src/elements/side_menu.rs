use super::*;

pub fn SideMenu() -> Element {
    let auth_state = use_auth_state().read_unchecked();
    let mut menu_state = use_menu_state();
    
    rsx! {
        if auth_state.has_role(ROLE_ADMINISTRATOR) {
            div { class: "divider my-0" }
            li {
                details {
                    summary {
                        Icon { icon: Icons::ShieldPerson, class: "size-8 sm:size-6 text-warning" }
                        { t!( "menu-administrator") }
                    }
                    ul {
                        MenuItem {
                            route: Route::Permissions {},
                            title: t!("menu-permissions"),
                            permission: PERMISSION_ROLES_READ,
                        }
                        MenuItem {
                            route: Route::Groups {},
                            title: t!("menu-groups"),
                            permission: PERMISSION_GROUPS_READ
                        }
                        MenuItem {
                            route: Route::Roles {},
                            title: t!("menu-roles"),
                            permission: PERMISSION_ROLES_READ
                        }
                        MenuItem {
                            route: Route::Users {},
                            title: t!("menu-users"),
                            permission: PERMISSION_USERS_READ
                        }
                    }
                }
            }
        }
        div { class: "divider my-0" }
        if auth_state.is_authenticated() {
            MenuItem {
                route: Route::ChangePassword {},
                title: t!("menu-settings"),
                Icon { icon: Icons::Settings, class: "size-8 sm:size-6" }
            }
            li {
                a {
                    onclick: move |event| {
                        use_search_engine_drop();
                        sign_out_task(event);
                        menu_state.set(false)
                    },
                    Icon { icon: Icons::SignOut, class: "size-8 sm:size-6 text-error" }
                    { t!("menu-sign-out") }
                }
            }
        } else {
            MenuItem {
                route: Route::SignIn {},
                title: t!("menu-sign-in"),
                Icon { icon: Icons::SignIn, class: "size-8 sm:size-6 text-accent" }
            }
        }
    }
}
