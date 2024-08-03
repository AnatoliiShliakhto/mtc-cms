use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use tracing::error;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::record_model::RecordModel;

use crate::APP_STATE;
use crate::component::main_menu_item::MainMenuItem;
use crate::component::modal_box::ModalBoxComponent;
use crate::element::footer::Footer;
use crate::element::header::Header;
use crate::handler::auth_handler::AuthHandler;
use crate::handler::schema_handler::SchemaHandler;
use crate::page::administrator::content::ContentPage;
use crate::page::administrator::dashboard::AdministratorDashboardPage;
use crate::page::administrator::editor::EditorPage;
use crate::page::administrator::groups::editor::GroupEditorPage;
use crate::page::administrator::groups::GroupsPage;
use crate::page::administrator::persons::PersonsPage;
use crate::page::administrator::roles::editor::RoleEditorPage;
use crate::page::administrator::roles::RolesPage;
use crate::page::administrator::schemas::editor::SchemaEditorPage;
use crate::page::administrator::schemas::SchemasPage;
use crate::page::administrator::users::editor::UserEditorPage;
use crate::page::administrator::users::UsersPage;
use crate::page::dashboard::DashboardPage;
use crate::page::home::HomePage;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Route {
    /// Core system routes
    #[layout(RootLayout)]
    #[route("/")]
    #[redirect("/:..segments", | segments: Vec < String > | Route::HomePage {})]
    HomePage {},
    #[route("/persons")]
    PersonsPage {},
    #[route("/administrator")]
    AdministratorDashboardPage {},
    #[route("/administrator/groups")]
    GroupsPage {},
    #[route("/administrator/groups/:group_prop")]
    GroupEditorPage { group_prop: String },
    #[route("/administrator/roles")]
    RolesPage {},
    #[route("/administrator/roles/:role_prop")]
    RoleEditorPage { role_prop: String },
    #[route("/administrator/users")]
    UsersPage {},
    #[route("/administrator/users/:user_prop")]
    UserEditorPage { user_prop: String },
    #[route("/administrator/schemas")]
    SchemasPage {},
    #[route("/administrator/schemas/:schema_prop")]
    SchemaEditorPage { schema_prop: String },
    #[route("/content/:schema_prop")]
    ContentPage { schema_prop: String },
    #[route("/editor/:schema_prop/:content_prop")]
    EditorPage { schema_prop: String, content_prop: String },
    #[route("/dashboard")]
    DashboardPage {},
}

#[component]
fn RootLayout() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();
    let mut main_menu_toggle = use_signal(|| false);

    let mut collections = use_signal(BTreeMap::<String, RecordModel>::new);

    use_effect(move || {
        let app_state = APP_STATE.peek();
        let auth_state = app_state.auth.read();

        if auth_state.is_auth() {
            spawn(async move {
                if let Ok(response) = APP_STATE.peek().api.get_all_collections().await {
                    let list = response
                        .list
                        .iter()
                        .map(|item| (item.title.clone(), item.clone()))
                        .collect::<BTreeMap<String, RecordModel>>();
                    collections.set(list)
                }
            });
        }
    });

    let sign_out = |_| {
        spawn(async move {
            let mut auth_state = APP_STATE.peek().auth.signal();

            match APP_STATE.peek().api.sign_out().await {
                Ok(auth_model) => auth_state.set(auth_model),
                Err(e) => error!("SignOut: {}", e.message()),
            }
        });
    };

    rsx! {
        div { class: "bg-base-100 drawer lg:drawer-open",
            input { class: "drawer-toggle",
                id: "main-menu",
                r#type: "checkbox",
                checked: main_menu_toggle(),
                onchange: move |event| main_menu_toggle.set(event.checked())
            }
            div { class: "drawer-content",
                Header {}
                div { class: "max-w-[100vw] px-6 pb-16 xl:pr-2",
                    div { class: "flex flex-col-reverse justify-between gap-6 xl:flex-row",
                        Outlet::<Route> {}
                            /*
                            div { class: "prose prose-sm md:prose-base w-full max-w-4xl flex-grow pt-10",
                                Outlet::<Route> {}
                            }
                            */
                    }
                }
            }
            div { class: "drawer-side z-[40]", style: "scroll-behavior: smooth; scroll-padding-top: 5rem;",
                label { class: "drawer-overlay", r#for: "main-menu" }
                aside { class: "flex flex-col bg-base-200 min-h-screen w-full sm:w-80",
                    div { class: "bg-base-200 sticky top-0 z-20 hidden items-center gap-2 bg-opacity-90 h-12 backdrop-blur lg:flex",
                        Link { class: "text-xl w-full btn btn-ghost", to: Route::HomePage {}, "MTC-CMS" }
                    }
                    div { class: "bg-base-200 sticky top-0 z-20 flex items-center gap-2 bg-opacity-90 h-12 backdrop-blur lg:hidden",
                        button { class: "w-full btn btn-ghost justify-start",
                            onclick: move |_| main_menu_toggle.set(false),
                            span { class: "flex flex-nowrap text-xl gap-5 items-center font-semibold",
                                Icon { class: "mt-[3px]",
                                    width: 26,
                                    height: 26,
                                    icon: dioxus_free_icons::icons::md_navigation_icons::MdArrowBack,
                                }
                                { translate!(i18, "messages.menu") }
                            }
                        }
                    }
                    ul { class: "menu menu-lg sm:menu-md",
                        if auth_state.is_permission("user::read") {
                            li {
                                Link { to: Route::PersonsPage {},
                                    onclick: move |_| main_menu_toggle.set(false),
                                    Icon { class: "text-accent",
                                        width: 20,
                                        height: 20,
                                        fill: "currentColor",
                                        icon: dioxus_free_icons::icons::md_social_icons::MdGroups
                                    }
                                    { translate!(i18, "messages.persons") }
                                }
                            }
                            div { class: "divider" }
                        }
                        if auth_state.is_permission("writer") {
                            li {
                                details {
                                    summary {
                                        Icon { class: "text-warning",
                                            width: 20,
                                            height: 20,
                                            fill: "currentColor",
                                            icon: dioxus_free_icons::icons::md_editor_icons::MdModeEdit
                                        }
                                        { translate!(i18, "messages.content") }
                                    }
                                    ul {
                                        MainMenuItem { route: Route::ContentPage { schema_prop: "singles".to_string() }, title: translate!(i18, "messages.singles"), rights: None, toggle: main_menu_toggle }
                                        li {
                                            details {
                                                summary {
                                                    { translate!(i18, "messages.collections") }
                                                }
                                                ul {
                                                    for (_, item) in collections() {
                                                        {
                                                            let m_schema = item.clone();
                                                            rsx! {
                                                                MainMenuItem {
                                                                    route: Route::ContentPage { schema_prop: m_schema.slug },
                                                                    title: m_schema.title,
                                                                    rights: None,
                                                                    toggle: main_menu_toggle
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
                        }
                        if auth_state.is_permission("administrator") {
                            div { class: "divider" }
                            li {
                                details {
                                    summary {
                                        Icon { class: "text-warning",
                                            width: 20,
                                            height: 20,
                                            fill: "currentColor",
                                            icon: dioxus_free_icons::icons::md_action_icons::MdAdminPanelSettings
                                        }
                                        { translate!(i18, "messages.administrator") }
                                    }
                                    ul {
                                        MainMenuItem { route: Route::SchemasPage {}, title: translate!(i18, "messages.schema"), rights: Some("schema::read".to_string()), toggle: main_menu_toggle }
                                        MainMenuItem { route: Route::GroupsPage {}, title: translate!(i18, "messages.groups"), rights: Some("group::read".to_string()), toggle: main_menu_toggle }
                                        MainMenuItem { route: Route::RolesPage {}, title: translate!(i18, "messages.roles"), rights: Some("role::read".to_string()), toggle: main_menu_toggle }
                                        MainMenuItem { route: Route::UsersPage {}, title: translate!(i18, "messages.users"), rights: Some("user::read".to_string()), toggle: main_menu_toggle }

                                    }
                                }
                            }
                        }
                        div { class: "divider" }
                        if auth_state.is_auth() {
                            li {
                                Link { to: Route::DashboardPage {},
                                    onclick: move |_| main_menu_toggle.set(false),
                                    Icon { class: "text-neutral",
                                        width: 20,
                                        height: 20,
                                        fill: "currentColor",
                                        icon: dioxus_free_icons::icons::md_action_icons::MdSettings
                                    }
                                    { translate!(i18, "messages.settings") }
                                }
                            }
                            li {
                                a {
                                    onclick: sign_out,
                                    Icon { class: "text-error",
                                        width: 20,
                                        height: 20,
                                        fill: "currentColor",
                                        icon: dioxus_free_icons::icons::md_action_icons::MdLogout
                                    }
                                    { translate!(i18, "messages.sign_out") }
                                }
                            }
                        } else {
                            li {
                                Link { to: Route::DashboardPage {},
                                    onclick: move |_| main_menu_toggle.set(false),
                                    Icon { class: "text-success",
                                        width: 20,
                                        height: 20,
                                        fill: "currentColor",
                                        icon: dioxus_free_icons::icons::md_action_icons::MdLogin
                                    }
                                    { translate!(i18, "messages.sign_in") }
                                }
                            }
                        }
                    }
                    div { class: "flex flex-col grow justify-end",
                        Footer {}
                    }
                }
            }
        }
        ModalBoxComponent {}
    }
}
