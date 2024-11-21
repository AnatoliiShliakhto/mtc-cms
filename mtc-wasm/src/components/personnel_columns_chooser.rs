use super::*;

#[component]
pub fn PersonnelColumnsChooser() -> Element {
    let columns = state_fn!(personnel_columns);
    let mut column_actions = columns.actions;
    let mut column_login = columns.login;
    let mut column_rank = columns.rank;
    let mut column_name = columns.name;
    let mut column_password = columns.password;
    let mut column_group = columns.group;
    let mut column_access = columns.access;

    rsx! {
        div {
            class: "fixed top-24 right-0 dropdown dropdown-left",
            div {
                tabindex: "0",
                role: "button",
                class: "btn rounded-l-lg rounded-r-none shadow-md hover:btn-accent",
                class: "opacity-50 xl:opacity-100 hover:opacity-100",
                Icon { icon: Icons::Columns, class: "size-6 m-1" }
            }
            div {
                tabindex: "0",
                class: "dropdown-content bg-base-100 rounded border input-bordered shadow-md \
                flex flex-col min-w-44 mr-2 gap-1 label-text p-3 z-[10]",
                label {
                    class: "cursor-pointer label justify-start gap-3",
                    input {
                        class: "checkbox checkbox-sm checkbox-info",
                        r#type: "checkbox",
                        initial_checked: column_actions(),
                        onchange: move |event| column_actions.set(event.checked())
                    }
                    span {
                        { t!("field-actions") }
                    }
                }
                label {
                    class: "cursor-pointer label justify-start gap-3",
                    input {
                        class: "checkbox checkbox-sm checkbox-info",
                        r#type: "checkbox",
                        initial_checked: column_login(),
                        onchange: move |event| column_login.set(event.checked())
                    }
                    span {
                        { t!("field-login") }
                    }
                }
                label {
                    class: "cursor-pointer label justify-start gap-3",
                    input {
                        class: "checkbox checkbox-sm checkbox-info",
                        r#type: "checkbox",
                        initial_checked: column_rank(),
                        onchange: move |event| column_rank.set(event.checked())
                    }
                    span {
                        { t!("field-rank") }
                    }
                }
                label {
                    class: "cursor-pointer label justify-start gap-3",
                    input {
                        class: "checkbox checkbox-sm checkbox-info",
                        r#type: "checkbox",
                        initial_checked: column_name(),
                        onchange: move |event| column_name.set(event.checked())
                    }
                    span {
                        { t!("field-short-name") }
                    }
                }
                label {
                    class: "cursor-pointer label justify-start gap-3",
                    input {
                        class: "checkbox checkbox-sm checkbox-info",
                        r#type: "checkbox",
                        initial_checked: column_password(),
                        onchange: move |event| column_password.set(event.checked())
                    }
                    span {
                        { t!("field-password") }
                    }
                }
                label {
                    class: "cursor-pointer label justify-start gap-3",
                    input {
                        class: "checkbox checkbox-sm checkbox-info",
                        r#type: "checkbox",
                        initial_checked: column_group(),
                        onchange: move |event| column_group.set(event.checked())
                    }
                    span {
                        { t!("field-group") }
                    }
                }
                label {
                    class: "cursor-pointer label justify-start gap-3",
                    input {
                        class: "checkbox checkbox-sm checkbox-info",
                        r#type: "checkbox",
                        initial_checked: column_access(),
                        onchange: move |event| column_access.set(event.checked())
                    }
                    span {
                        { t!("field-access") }
                    }
                }
            }
        }
    }
}