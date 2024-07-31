use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use tracing::error;

use mtc_model::auth_model::AuthModelTrait;

use crate::component::main_menu_item::MainMenuItem;
use crate::component::modal_box::ModalBoxComponent;
use crate::element::header::Header;
use crate::handler::auth_handler::AuthHandler;
use crate::page::administrator::groups::editor::GroupEditorPage;
use crate::page::administrator::groups::GroupsPage;
use crate::page::administrator::persons::PersonsPage;
use crate::page::administrator::roles::editor::RoleEditorPage;
use crate::page::administrator::roles::RolesPage;
use crate::page::administrator::schemas::editor::SchemaEditorPage;
use crate::page::administrator::schemas::SchemasPage;
use crate::page::administrator::users::editor::UserEditorPage;
use crate::page::administrator::users::UsersPage;
use crate::page::administrator::AdministratorPage;
use crate::page::dashboard::DashboardPage;
use crate::page::home::HomePage;
use crate::page::instructor::InstructorPage;
use crate::APP_STATE;
use crate::element::footer::Footer;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Route {
    #[layout(RootLayout)]
    #[route("/")]
    #[redirect("/:..segments", | segments: Vec < String > | Route::HomePage {})]
    HomePage {},
    #[route("/persons")]
    PersonsPage {},
    #[route("/administrator")]
    AdministratorPage {},
    #[route("/administrator/groups")]
    GroupsPage {},
    #[route("/administrator/groups/:group")]
    GroupEditorPage { group: String },
    #[route("/administrator/roles")]
    RolesPage {},
    #[route("/administrator/roles/:role")]
    RoleEditorPage { role: String },
    #[route("/administrator/users")]
    UsersPage {},
    #[route("/administrator/users/:user")]
    UserEditorPage { user: String },
    #[route("/administrator/schemas")]
    SchemasPage {},
    #[route("/administrator/schemas/:schema")]
    SchemaEditorPage { schema: String },
    #[route("/persons")]
    InstructorPage {},
    #[route("/dashboard")]
    DashboardPage {},
}

#[component]
fn RootLayout() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();
    let mut main_menu_toggle = use_signal(|| false);

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
                aside { class: "bg-base-200 min-h-screen w-full sm:w-80",
                    div { class: "bg-base-200 sticky top-0 z-20 hidden items-center gap-2 bg-opacity-90 h-12 backdrop-blur lg:flex",
                        Link { class: "text-xl w-full btn btn-ghost", to: Route::HomePage {}, "MTC-CMS" }
                    }
                    div { class: "bg-base-200 sticky top-0 z-20 flex items-center gap-2 bg-opacity-90 h-12 backdrop-blur lg:hidden",
                        button { class: "w-full btn btn-ghost justify-start",
                            onclick: move |_| main_menu_toggle.set(false),
                            span { class: "flex flex-nowrap text-2xl gap-3 items-center", style: "font-weight: 600;",
                                Icon {
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
                        }
                        if auth_state.is_permission("administrator") {
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
                    div { class: "flex flex-col h-full justify-end",
                        Footer {}
                    }    
                }
            }
        }
    //        Footer {}
        ModalBoxComponent {}
    }
}
