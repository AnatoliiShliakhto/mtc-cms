use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::group_model::{GroupModel, GroupsModel};
use mtc_model::pagination_model::PaginationModel;

use crate::APP_STATE;
use crate::component::paginator::{PaginatorComponent, PaginatorComponentMode};
use crate::handler::group_handler::GroupHandler;

#[derive(Props, Clone, PartialEq)]
pub struct GroupListProps {
    pub page: Signal<usize>,
    pub selected: Signal<i32>,
}

pub fn GroupList(mut props: GroupListProps) -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();
    let i18 = use_i18();

    let groups = use_context::<Signal<BTreeMap<i32, GroupModel>>>();
    let pagination = use_context::<Signal<PaginationModel>>();
    let mut is_busy = use_signal(|| false);

    let delete_groups = move |event: Event<FormData>| {
        event.stop_propagation();
        if let Some((&_, value)) = event.values().get_key_value("groups") {
            is_busy.set(true);
            let groups_to_delete = GroupsModel { groups: value.0.to_vec().to_owned() };
            spawn(async move {
                if APP_STATE.peek().api.delete_group_list(groups_to_delete).await.is_ok() {
                    props.page.set(pagination().current_page);
                }
                is_busy.set(false);
            });
        }
    };

    rsx! {
        div { class: "flex flex-row py-3",
            div { class: "flex flex-col gap-3 grow items-center pr-5",
                form { class: "flex w-full", 
                    id: "groups-form",
                    onsubmit: delete_groups,

                    table { class: "table w-full",
                        thead {
                            tr {
                                th { style: "width: 1.75rem;" }
                                th { { translate!(i18, "messages.slug") } }
                                th { { translate!(i18, "messages.title") } }
                            }
                        }
                        tbody {
                            for (id, item) in groups.read_unchecked().to_owned() {
                                tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                    onclick: move |event| {
                                        event.stop_propagation();
                                        props.selected.set(id)
                                    },

                                    td {
                                        onclick: move |event| event.stop_propagation(),
                                        input { class: "checkbox-xs",
                                            r#type: "checkbox",
                                            name: "groups",
                                            value: item.slug.clone(),
                                        }
                                    }
                                    td { { item.slug.clone() } }
                                    td { { item.title } }
                                }
                            }
                        }
                    }
                }
                PaginatorComponent { mode: PaginatorComponentMode::Full, page: props.page, pagination }
            }
            div { class: "flex flex-col gap-3 min-w-36",
                div { class: "flex flex-wrap gap-3",
                    PaginatorComponent { mode: PaginatorComponentMode::Compact, page: props.page, pagination }
                }
                if auth_state.is_permission("group::write") && !is_busy() {
                    button { class: "w-full btn btn-outline btn-accent gap-3 justify-start",
                        prevent_default: "onclick",
                        onclick: move |_| props.selected.set(0),
                        Icon {
                            width: 16,
                            height: 16,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::fa_regular_icons::FaSquarePlus
                        }
                        { translate!(i18, "messages.add") }
                    }
                }
                if auth_state.is_permission("group::delete") && !is_busy() {
                    button { class: "w-full btn btn-outline btn-error gap-3 justify-start",
                        r#type: "submit",
                        prevent_default: "onsubmit onclick",
                        form: "groups-form",
                        Icon {
                            width: 16,
                            height: 16,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::fa_regular_icons::FaTrashCan
                        }
                        { translate!(i18, "messages.delete") }
                    }
                }
                if is_busy() {
                    span { class: "loading loading-bars loading-lg p-3 self-center" }
                }
            }
        }
    }
}