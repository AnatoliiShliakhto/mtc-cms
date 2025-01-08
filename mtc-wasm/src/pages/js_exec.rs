use super::*;

/// A component that allows to execute arbitrary JavaScript code.
///
/// This component is accessible only for users with [`ROLE_ADMINISTRATOR`].
/// It shows a text area where user can input JavaScript code to be executed.
/// The code is executed on form submit and the result is displayed in
/// another text area below the input text area.
///
/// The component is useful for debugging and testing purposes.
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