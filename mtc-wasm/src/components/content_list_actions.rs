use super::*;

#[component]
pub fn ContentListActions<T: PartialEq + Clone + 'static>(
    #[props]
    future: Resource<T>,
    #[props(into)]
    schema: String,
) -> Element {
    let schema = use_memo(use_reactive!(|schema| schema));

    let mut is_show = use_signal(|| false);

    let submit = move |event: Event<FormData>| {
        let schema = event.get_str("schema").unwrap_or_default();
        let slug = event.get_str("slug").unwrap_or_default();
        let payload = json!({
            "slug": slug.clone(),
            "title": event.get_str("title")
        });

        spawn(async move {
            if post_request!(url!(API_CONTENT, &schema, ID_CREATE), payload) {
                navigator().push(route!(API_EDITOR, &schema, &slug));
            }
            is_show.set(false);
        });
    };

    rsx! {
        div {
            class: "action-right-panel top-24 group join join-vertical \
            opacity-50 xl:opacity-100 hover:opacity-100",
            onclick: move |event| event.stop_propagation(),
            button {
                class: "hover:btn-primary join-item",
                onclick: move |_| future.restart(),
                Icon { icon: Icons::Refresh, class: "size-8" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-refresh") }
                }
            }
            button {
                class: "hover:btn-accent join-item",
                onclick: move |_| is_show.set(true),
                Icon { icon: Icons::Plus, class: "size-8" }
                span {
                    class: "opacity-0 group-hover:opacity-100",
                    { t!("action-create") }
                }
            }
        }
        if is_show() {
            section {
                class: "modal modal-open",
                onclick: move |_| is_show.set(false),

                div {
                    class: "modal-box min-w-96 w-fit",
                    onclick: move |event| event.stop_propagation(),
                    Icon {
                        icon: Icons::Description,
                        class: "size-6 absolute top-6 left-6 text-neutral"
                    }
                    div {
                        class: "absolute top-0 right-0 join rounded-none",
                        button {
                            class: "btn btn-sm btn-ghost join-item hover:text-error",
                            onclick: move |_| is_show.set(false),
                            Icon { icon: Icons::Close, class: "size-4" }
                        }
                    }
                    h1 {
                        class: "text-title text-lg text-center",
                            { t!("caption-create-page") }
                    }
                    div { class: "divider my-0" }
                    form {
                        class: "flex grow flex-col items-center gap-3",
                        id: "content-create-form",
                        autocomplete: "off",
                        onsubmit: submit,
                        input {
                            r#type: "hidden",
                            name: "schema",
                            value: schema()
                        }
                        FormTextField {
                            name: "slug",
                            title: "field-slug",
                            pattern: SLUG_PATTERN,
                            required: true,
                        }
                        FormTextField {
                            name: "title",
                            title: "field-title",
                            pattern: TITLE_PATTERN,
                            required: true,
                        }
                    }
                    div {
                        class: "card-actions mt-6 gap-6 justify-end",
                        button {
                            form: "content-create-form",
                            class: "btn btn-primary",
                            { t!("action-yes") }
                        }
                        button {
                            class: "btn btn-outline",
                            onclick: move |_| is_show.set(false),
                            { t!("action-no") }
                        }
                    }
                }
            }
        }
    }
}