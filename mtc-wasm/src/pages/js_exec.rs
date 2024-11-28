use super::*;

#[component]
pub fn JsExec() -> Element {
    breadcrumbs!("menu-administrator");
    check_role!(ROLE_ADMINISTRATOR);

    let submit = move |event: Event<FormData>| {
        let js_eval =
            event.get_str("js-text").unwrap_or_default();
        eval(&js_eval);
    };

    let script = r#"try {

    const error = 'Your error here!'
    throw new Error(error);
} catch (err) {
    document.getElementById('js-output').innerText = err;
}"#;

    rsx! {
        div {
            class: "w-full sm:px-10",
            form {
                class: "w-full",
                autocomplete: "off",
                onsubmit: submit,

                textarea {
                    name: "js-text",
                    class: "w-full h-96 rounded textarea textarea-bordered",
                    initial_value: script
                }
                button {
                    class: "btn btn-primary my-10",
                    { t!("action-execute")}
                }
            }
            textarea {
                class: "textarea w-full rounded textarea textarea-bordered",
                id: "js-output"
            }
        }
    }
}