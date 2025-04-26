use super::*;

#[component]
pub fn Administrator() -> Element {
    breadcrumbs!("menu-administrator");
    check_role!(ROLE_ADMINISTRATOR);

    let future = value_future!(url!(API_SYSTEM));
    let response = future.suspend()?;
    check_response!(response, future);

    let system_info = response().key_obj::<SystemInfo>("info").unwrap_or_default();
    let last_migration = if let Some(migration) = response()
        .key_obj::<Vec<Cow<'static, str>>>("migrations").unwrap_or_default().last() {
        migration.clone()
    } else { "0000-init.surql".into() };

    let groups_stat = response().key_obj::<Vec<GroupStat>>("groups_stat").unwrap_or_default();
    let groups_stat_total = format!("{} / {}",
        groups_stat.iter().map(|x| x.online).sum::<i64>(),
        groups_stat.iter().map(|x| x.total).sum::<i64>()
    );

    let migrate = move |event: Event<MouseData>| {
        spawn(async move {
            let dummy = Value::Null;
            if post_request!(url!(API_MIGRATE), dummy) {
                success_dialog!("message-success-migration")
            }
        });
    };

    let rebuild = move |event: Event<MouseData>| {
        spawn(async move {
            if post_request!(url!(API_IDX_REBUILD)) {
                success_dialog!("message-success-index-rebuild")
            }
        });
    };

    let sitemap = move |event: Event<MouseData>| {
        spawn(async move {
            if post_request!(url!(API_SITEMAP)) {
                success_dialog!("message-success-sitemap-build")
            }
        });
    };

    let course_assets = move |event: Event<MouseData>| {
        spawn(async move {
            if post_request!(url!("system/courses")) {
                success_dialog!("message-success-course-assets-build")
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
                        div {
                            class: "stat-actions",
                            button {
                                class: "btn btn-sm",
                                onclick: course_assets,
                                { t!("action-index") }
                            }
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

                div {
                    class: "stats w-72 shadow-md",
                    div {
                        class: "stat",
                        div {
                            class: "stat-figure text-info",
                            Icon { icon: Icons::Map, class: "size-12" }
                        }
                        div {
                            class: "stat-title",
                            { t!("message-stat-sitemap-title") }
                        }
                        div {
                            class: "stat-value proportional-nums",
                            { response().key_i64("sitemap").unwrap_or_default().to_string() }
                        }
                        div {
                            class: "stat-desc",
                            { t!("message-stat-sitemap-description") }
                        }
                        div {
                            class: "stat-actions",
                            button {
                                class: "btn btn-sm",
                                onclick: sitemap,
                                { t!("action-create") }
                            }
                        }
                    }
                }

            }
            div {
                class: "flex grow flex-wrap gap-5",
                div {
                    class: "stats shadow-md",
                    div {
                        class: "stat",
                        div {
                            class: "stat-figure text-info",
                            Icon { icon: Icons::Personnel, class: "size-12" }
                        }
                        div {
                            class: "stat-title",
                            { t!("message-stat-groups-title") }
                        }
                        div {
                            class: "stat-desc",
                            table {
                                class: "table table-sm w-full",
                                for group in groups_stat.into_iter() {
                                    tr {
                                        td {
                                            { group.title }
                                        }
                                        td {
                                            { format!("{} / {}", group.online, group.total) }
                                        }
                                    }
                                }
                                tr {
                                    td {
                                        class: "font-semibold text-info",
                                        { "TOTAL" }
                                    }
                                    td {
                                        class: "font-semibold text-info",
                                        { groups_stat_total }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}