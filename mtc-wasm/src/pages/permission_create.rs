use super::*;

pub fn PermissionCreate() -> Element {
    let auth_state = use_auth_state();
    let api = use_coroutine_handle::<ApiRequestAction>();

    page_init!("menu-permissions", PERMISSION_ROLES_WRITE, auth_state);
    
    rsx! {
        section { 
            class: "flex grow select-none flex-row px-3 gap-6",
            form { 
                class: "flex grow flex-col items-center gap-3",
                id: "permission-form",
                autocomplete: "off",
                onsubmit: move |event| {
                    let Some(slug) = event.get_str("slug") else { return };
                    api.send(ApiRequestAction::PostThenBack(
                        url!("permission", &slug),
                        None
                    ))
                },

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
                        onclick: move |_| navigator().go_back(),
                        Icon { icon: Icons::Cancel, class: "size-6" }
                        { t!("action-cancel") }
                    }
                }
            }
        }         
    }
}