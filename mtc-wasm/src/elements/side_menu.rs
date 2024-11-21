use super::*;

#[component]
pub fn SideMenu() -> Element {
    let auth = state!(auth);

    rsx! {
        Menu {}
        if auth.has_role(ROLE_WRITER) {
            div { class: "divider my-0" }
            li {
                details {
                    summary {
                        Icon { icon: Icons::Pen, class: "size-7 sm:size-5 text-warning" }
                        { t!( "menu-content") }
                    }
                    ul {
                        MenuItem {
                            route: route!(API_CONTENT, API_PAGE),
                            permission: PERMISSION_SCHEMAS_READ,
                            { t!("menu-page") }
                        }
                        li {
                            details {
                                summary {
                                    { t!( "menu-pages") }
                                }
                                ul {
                                    for page in state!(pages) {
                                        MenuItem {
                                            route: route!(API_CONTENT, page.slug),
                                            { page.title }
                                        }
                                    }
                                }
                            }
                        }
                        MenuItem {
                            route: route!(API_CONTENT, API_COURSE),
                            permission: PERMISSION_SCHEMAS_READ,
                            { t!("menu-course") }
                        }
                    }
                }
            }
        }
        if auth.has_role(ROLE_ADMINISTRATOR) {
            div { class: "divider my-0" }
            li {
                details {
                    summary {
                        Icon { icon: Icons::ShieldPerson, class: "size-8 sm:size-6 text-warning" }
                        { t!( "menu-administrator") }
                    }
                    ul {
                        MenuItem {
                            route: route!(API_ADMINISTRATOR, API_SCHEMAS),
                            permission: PERMISSION_SCHEMAS_READ,
                            { t!("menu-schemas") }
                        }
                        MenuItem {
                            route: route!(API_ADMINISTRATOR, API_PERMISSIONS),
                            permission: PERMISSION_ROLES_READ,
                            { t!("menu-permissions") }
                        }
                        MenuItem {
                            route: route!(API_ADMINISTRATOR, API_GROUPS),
                            permission: PERMISSION_GROUPS_READ,
                            { t!("menu-groups") }
                        }
                        MenuItem {
                            route: route!(API_ADMINISTRATOR, API_ROLES),
                            permission: PERMISSION_ROLES_READ,
                            { t!("menu-roles") }
                        }
                        MenuItem {
                            route: route!(API_ADMINISTRATOR, API_USERS),
                            permission: PERMISSION_USERS_READ,
                            { t!("menu-users") }
                        }
                    }
                }
            }
        }
        div { class: "divider my-0" }
        if auth.is_authenticated() {
            MenuItem {
                route: route!(API_PERSONNEL),
                permission: PERMISSION_USERS_READ,
                Icon { icon: Icons::Personnel, class: "size-8 sm:size-6 text-info" }
                { t!("menu-personnel") }
            }
            li {
                details {
                    summary {
                        Icon { icon: Icons::Settings, class: "size-8 sm:size-6 text-neutral" }
                        { t!( "menu-settings") }
                    }
                    ul {
                        MenuItem {
                            route: route!(API_AUTH, "change-password"),
                            { t!("menu-change-password") }
                        }
                        MenuItem {
                            route: route!(API_AUTH, "linking-qr-code"),
                            { t!("menu-linking-qr-code") }
                        }
                    }
                }
            }
            li {
                a {
                    onclick: move |event| {
                        state_fn!(search_engine_clear);
                        sign_out_task(event);
                        state!(set_menu, false)
                    },
                    Icon { icon: Icons::SignOut, class: "size-8 sm:size-6 text-error" }
                    { t!("menu-sign-out") }
                }
            }
        } else {
            MenuItem {
                route: route!(API_AUTH, API_SIGN_IN),
                Icon { icon: Icons::SignIn, class: "size-8 sm:size-6 text-accent" }
                { t!("menu-sign-in") }
            }
        }
    }
}
