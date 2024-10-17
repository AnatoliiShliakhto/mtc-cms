use super::*;

#[component]
pub fn PermissionCreate() -> Element {
    breadcrumbs!("menu-permissions");
    check_permission!(PERMISSION_ROLES_WRITE);

    let submit = move |event: Event<FormData>| {
        let Some(slug) = event.get_str("slug") else { return };

        spawn(async move {
            if post_request!(url!(API_PERMISSION, &slug)) {
                navigator().replace(Route::Permissions {});
            }
        });
    };

    rsx! {
        section { 
            class: "flex grow select-none flex-row px-3 gap-6",
            form { 
                class: "flex grow flex-col items-center gap-3",
                id: "permission-form",
                autocomplete: "off",
                onsubmit: submit,

                FormTextField {
                    name: "slug",
                    title: "field-slug",
                    pattern: SLUG_PATTERN,
                    required: true
                }

                div { 
                    class: "flex p-2 gap-5 flex-inline",
                    button {
                        class: "btn btn-primary",
                        r#type: "submit",
                        Icon { icon: Icons::Plus, class: "size-6" }
                        { t!("action-create") }
                    }
                    button { 
                        class: "btn btn-ghost text-error",
                        onclick: move |_| {
                            navigator().replace(Route::Permissions {});
                        },
                        Icon { icon: Icons::Cancel, class: "size-6" }
                        { t!("action-cancel") }
                    }
                }
            }
        }         
    }
}