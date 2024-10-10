use super::*;

pub fn Administrator() -> Element {
    build_breadcrumbs("menu-administrator");

    let auth_state = use_auth_state()();
    if !auth_state.is_admin() {
        return rsx! { AccessForbidden {} }
    }

    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let api_client = use_api_client();

    let mut future =
        use_resource(move || async move {
            request_fetch_task(url!(API_SYSTEM)).await
        });

    let response = future.suspend()?;
    if response().is_null() { fail!(future) }

    let system_info = response().get_object::<SystemInfo>("info").unwrap_or_default();
    let last_migration = if let Some(migration) = response()
        .get_str_array("migrations").unwrap_or_default().last() {
        migration.clone()
    } else { "0000-Init.sql".into() };

    let migrate = move |event: Event<MouseData>| {
        let url: Cow<'static, str> = url!(API_MIGRATE);
        spawn(async move {
            match api_client()
                .post(&*url)
                .json(&json!({}))
                .send()
                .await
                .consume()
                .await {
                Ok(_) => {
                    message_box_task
                        .send(MessageBoxAction::Success(t!("message-success-post")));
                    future.restart()
                },
                Err(e) =>
                    message_box_task.send(MessageBoxAction::Error(e.message())),
            }
        });
    };

    let rebuild = move |event: Event<MouseData>| {
        let url: Cow<'static, str> = url!(API_IDX_REBUILD);
        spawn(async move {
            match api_client()
                .get(&*url)
                .send()
                .await
                .consume()
                .await {
                Ok(_) => {
                    message_box_task
                        .send(MessageBoxAction::Success(t!("message-success-post")));
                    future.restart()
                },
                Err(e) =>
                    message_box_task.send(MessageBoxAction::Error(e.message())),
            }
        });
    };

    rsx! {
        section {
            class: "flex grow select-none flex-col gap-6 px-3 pr-20 sm:pr-16",
            h3 {
                class: "flex w-full flex-wrap pb-4 sm:px-4 justify-center text-2xl font-semibold",
                { t!("caption-administrator-dashboard") }
            }
            div {
                class: "flex grow flex-wrap gap-5",

                div {
                    class: "stats w-72 shadow-md",
                    div {
                        class: "stat",
                        div {
                            class: "stat-figure text-success",
                            Icon { icon: Icons::Group, class: "size-12" }
                        }
                        div {
                            class: "stat-title",
                            { t!("message-stat-users-title") }
                        }
                        div {
                            class: "stat-value proportional-nums",
                            { system_info.active_users.to_string() }
                        }
                        div {
                            class: "stat-desc",
                            { t!("message-stat-users-description") }
                        }
                    }
                }

                div {
                    class: "stats w-72 shadow-md",
                    div {
                        class: "stat",
                        div {
                            class: "stat-figure text-secondary",
                            Icon { icon: Icons::Group, class: "size-12" }
                        }
                        div {
                            class: "stat-title",
                            { t!("message-stat-total-users-title") }
                        }
                        div {
                            class: "stat-value proportional-nums",
                            { system_info.users.to_string() }
                        }
                        div {
                            class: "stat-desc",
                            { t!("message-stat-total-users-description") }
                        }
                    }
                }

                div {
                    class: "stats w-72 shadow-md",
                    div {
                        class: "stat",
                        div {
                            class: "stat-figure",
                            Icon { icon: Icons::Description, class: "size-12" }
                        }
                        div {
                            class: "stat-title",
                            { t!("message-stat-pages-title") }
                        }
                        div {
                            class: "stat-value proportional-nums",
                            { system_info.pages.to_string() }
                        }
                        div {
                            class: "stat-desc",
                            { t!("message-stat-pages-description") }
                        }
                    }
                }

                div {
                    class: "stats w-72 shadow-md",
                    div {
                        class: "stat",
                        div {
                            class: "stat-figure text-accent",
                            Icon { icon: Icons::Download, class: "size-12" }
                        }
                        div {
                            class: "stat-title",
                            { t!("message-stat-downloads-title") }
                        }
                        div {
                            class: "stat-value proportional-nums",
                            { system_info.files.to_string() }
                        }
                        div {
                            class: "stat-desc",
                            { t!("message-stat-downloads-description") }
                        }
                    }
                }

                div {
                    class: "stats w-72 shadow-md",
                    div {
                        class: "stat",
                        div {
                            class: "stat-figure text-error",
                            Icon { icon: Icons::Camera, class: "size-12" }
                        }
                        div {
                            class: "stat-title",
                            { t!("message-stat-media-title") }
                        }
                        div {
                            class: "stat-value proportional-nums",
                            { system_info.media.to_string() }
                        }
                        div {
                            class: "stat-desc",
                            { t!("message-stat-media-description") }
                        }
                    }
                }

                div {
                    class: "stats w-72 shadow-md",
                    div {
                        class: "stat",
                        div {
                            class: "stat-figure text-primary",
                            Icon { icon: Icons::Link45Deg, class: "size-12" }
                        }
                        div {
                            class: "stat-title",
                            { t!("message-stat-links-title") }
                        }
                        div {
                            class: "stat-value proportional-nums",
                            { system_info.links.to_string() }
                        }
                        div {
                            class: "stat-desc",
                            { t!("message-stat-links-description") }
                        }
                    }
                }

                div {
                    class: "stats w-72 shadow-md",
                    div {
                        class: "stat",
                        div {
                            class: "stat-figure text-success",
                            Icon { icon: Icons::Diagram3, class: "size-12" }
                        }
                        div {
                            class: "stat-title",
                            { t!("message-stat-courses-title") }
                        }
                        div {
                            class: "stat-value proportional-nums",
                            { system_info.courses.to_string() }
                        }
                        div {
                            class: "stat-desc",
                            { t!("message-stat-courses-description") }
                        }
                    }
                }

                div {
                    class: "stats w-72 shadow-md",
                    div {
                        class: "stat",
                        div {
                            class: "stat-figure text-warning",
                            Icon { icon: Icons::Database, class: "size-12" }
                        }
                        div {
                            class: "stat-title",
                            { t!("message-stat-database-title") }
                        }
                        div {
                            class: "stat-value text-lg",
                            { last_migration }
                        }
                        div {
                            class: "stat-desc",
                            { t!("message-stat-database-description") }
                        }
                        div {
                            class: "stat-actions",
                            button {
                                class: "btn btn-sm",
                                onclick: migrate,
                                { t!("action-migrate") }
                            }
                        }
                    }
                }

                div {
                    class: "stats w-72 shadow-md",
                    div {
                        class: "stat",
                        div {
                            class: "stat-figure text-accent",
                            Icon { icon: Icons::Search, class: "size-12" }
                        }
                        div {
                            class: "stat-title",
                            { t!("message-stat-idx-title") }
                        }
                        div {
                            class: "stat-value proportional-nums",
                            { system_info.indexes.to_string() }
                        }
                        div {
                            class: "stat-desc",
                            { t!("message-stat-idx-description") }
                        }
                        div {
                            class: "stat-actions",
                            button {
                                class: "btn btn-sm",
                                onclick: rebuild,
                                { t!("action-index") }
                            }
                        }
                    }
                }

            }
        }
    }
}