use super::*;

pub fn Home() -> Element {
    breadcrumbs!();

    rsx! {
        InitBox {}
    }
}