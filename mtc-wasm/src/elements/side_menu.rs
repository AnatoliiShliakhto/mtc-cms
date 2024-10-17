use super::*;

#[component]
pub fn SideMenu() -> Element {
    let auth_state = &*use_auth_state().read_unchecked();
    let mut menu_state = use_menu_state();

    rsx! {
        Menu {}
        if auth_state.has_role(ROLE_WRITER) {
            div { class: "divider my-0" }
            li {
                details {
                    summary {
                        Icon { icon: Icons::Pen, class: "size-7 sm:size-5 text-warning" }
                        { t!( "menu-content") }
                    }
                    ul {
                        MenuItem {
                            route: Route::ContentList { schema: "page".to_string() },
                            permission: PERMISSION_SCHEMAS_READ,
                            { t!("menu-page") }
                        }
                        li {
                            details {
                                summary {
                                    { t!( "menu-pages") }
                                }
                                ul {
                                    for page in use_pages_entries()() {
                                        MenuItem {
                                            route: Route::ContentList {
                                                schema: page.slug.to_string()
                                            },
                                            { page.title }
                                        }
                                    }
                                }
                            }
                        }
                        MenuItem {
                            route: Route::ContentList { schema: "course".to_string() },
                            permission: PERMISSION_SCHEMAS_READ,
                            { t!("menu-course") }
                        }
                    }
                }
            }
        }
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
                            route: Route::Schemas {},
                            permission: PERMISSION_SCHEMAS_READ,
                            { t!("menu-schemas") }
                        }
                        MenuItem {
                            route: Route::Permissions {},
                            permission: PERMISSION_ROLES_READ,
                            { t!("menu-permissions") }
                        }
                        MenuItem {
                            route: Route::Groups {},
                            permission: PERMISSION_GROUPS_READ,
                            { t!("menu-groups") }
                        }
                        MenuItem {
                            route: Route::Roles {},
                            permission: PERMISSION_ROLES_READ,
                            { t!("menu-roles") }
                        }
                        MenuItem {
                            route: Route::Users {},
                            permission: PERMISSION_USERS_READ,
                            { t!("menu-users") }
                        }
                    }
                }
            }
        }
        div { class: "divider my-0" }
        if auth_state.is_authenticated() {
            MenuItem {
                route: Route::ChangePassword {},
                Icon { icon: Icons::Settings, class: "size-8 sm:size-6 text-neutral" }
                { t!("menu-settings") }
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
                Icon { icon: Icons::SignIn, class: "size-8 sm:size-6 text-accent" }
                { t!("menu-sign-in") }
            }
        }
    }
}
