use super::*;

pub fn Home() -> Element {
    drop_breadcrumbs();

    rsx!{ 
        InitBox {}
    }
}