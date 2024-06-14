use dioxus::prelude::*;

use mtc_model::pagination_model::PaginationModel;

use crate::component::paginator::compact::PaginatorCompact;
use crate::component::paginator::full::PaginatorFull;

mod compact;
mod full;

#[derive(Props, Clone, PartialEq)]
pub struct PaginatorComponentProps {
    pub mode: PaginatorComponentMode,
    pub page: Signal<usize>,
    pub pagination: PaginationModel,
}

#[derive(Clone, PartialEq)]
pub enum PaginatorComponentMode {
    Compact,
    Full,
}

pub fn PaginatorComponent(props: PaginatorComponentProps) -> Element {
    match props.mode {
        PaginatorComponentMode::Compact => PaginatorCompact(props.page, props.pagination),
        PaginatorComponentMode::Full => PaginatorFull(props.page, props.pagination),
    }
}