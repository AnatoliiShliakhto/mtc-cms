use super::*;

/// Component for adding a new personnel.
///
/// The component shows a form with three fields: `login`, `rank` and `name`.
#[component]
pub fn PersonnelAdd() -> Element {
    breadcrumbs!("menu-personnel");
    check_permission!(PERMISSION_USERS_READ);

    let mut users = state_fn!(personnel);

    let submit = move |event: Event<FormData>| {
        let login = event.get_str("login")
            .unwrap_or_default().to_uppercase().replace(" ", "");
        users.write().insert(login.clone().into(), UserDetails {
            login: login.into(),
            rank: event.get_str("rank").unwrap_or_default().trim().to_string().to_lowercase().into(),
            name: event.get_str("name").unwrap_or_default().trim().to_string().into(),
            ..Default::default()
        });
        navigator().replace(route!(API_PERSONNEL));
    };

    rsx! {
        section {
            class: "flex grow select-none flex-row px-3 gap-6",
            form {
                class: "flex grow flex-col items-center gap-3",
                id: "personnel-form",
                autocomplete: "off",
                onsubmit: submit,

                FormTextField {
                    name: "login",
                    title: "field-login",
                    required: true
                }
                FormTextField {
                    name: "rank",
                    title: "field-rank",
                    required: true
                }
                FormTextField {
                    name: "name",
                    title: "field-name",
                    required: true
                }

                div {
                    class: "flex p-2 gap-5 flex-inline",
                    button {
                        class: "btn btn-primary",
                        r#type: "submit",
                        Icon { icon: Icons::Plus, class: "size-6" }
                        { t!("action-add") }
                    }
                    button {
                        class: "btn btn-ghost text-error",
                        onclick: move |_| {
                            navigator().replace(route!(API_PERSONNEL));
                        },
                        Icon { icon: Icons::Cancel, class: "size-6" }
                        { t!("action-cancel") }
                    }
                }
            }
        }
    }
}