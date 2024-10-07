use super::*;

pub fn SideMenu() -> Element {
    let auth_state = &*use_auth_state().read_unchecked();
    let mut menu_state = use_menu_state();
    
    rsx! {
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
                            title: t!("menu-page"),
                            permission: PERMISSION_SCHEMAS_READ,
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
                                            title: page.title,
                                        }
                                    }
                                }
                            }
                        }
                        MenuItem {
                            route: Route::ContentList { schema: "course".to_string() },
                            title: t!("menu-course"),
                            permission: PERMISSION_SCHEMAS_READ,
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
                            title: t!("menu-schemas"),
                            permission: PERMISSION_SCHEMAS_READ,
                        }
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
