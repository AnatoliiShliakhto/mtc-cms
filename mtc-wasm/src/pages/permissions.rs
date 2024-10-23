use super::*;

#[component]
pub fn Permissions() -> Element {
    let auth = use_auth_state();

    breadcrumbs!("menu-permissions");
    check_permission!(PERMISSION_ROLES_READ);

    let mut future = value_future!(url!(API_PERMISSIONS));
    let response = future.suspend()?;
    check_response!(response, future);

    let delete = move |slug: &str| {
        let slug = slug.to_owned();
        spawn(async move {
            if delete_request!(url!(API_PERMISSION, &slug)) {
                success_dialog!("message-success-deletion");
                future.restart()
            }
        });
    };

    rsx! {
        section { 
            class: "w-full flex-grow xl:pr-16",
            table { 
                class: "entry-table",
                thead {
                    tr {
                        th { class: "w-12" }
                        th { { t!("field-slug") } }
                        th { { t!("field-read") } }
                        th { { t!("field-write") } }
                        th { { t!("field-delete") } }
                    }
                }
                tbody {

                    for permission in response()
                    .self_obj::<Vec<Cow<'static, str>>>()
                    .unwrap_or_default().iter() {{
                        let slug = permission.to_owned();
                        rsx! {
                            tr {
                                td {
                                    if auth().has_permission(PERMISSION_ROLES_DELETE) {
                                        button {
                                            class: "btn btn-xs btn-ghost",
                                            onclick: move |_| delete(&slug),
                                            Icon { icon: Icons::Close, class: "size-4 text-error" }
                                        }
                                    }
                                }
                                td {
                                    { permission }
                                }
                                td {
                                    class: "text-neutral",
                                    { permission } { "::read" }
                                }
                                td {
                                    class: "text-neutral",
                                    { permission } { "::write" }
                                }
                                td {
                                    class: "text-neutral",
                                    { permission } { "::delete" }
                                }
                            }
                        }
                    }}

                }
                EntriesActions {
                    future,
                    route: route!(API_ADMINISTRATOR, API_PERMISSION, ID_CREATE),
                    permission: PERMISSION_ROLES_WRITE,
                }
            }
        }
    }
}