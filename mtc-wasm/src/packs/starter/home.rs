use super::*;

/// The `Home` function renders the home page of the application.
///
/// This function initializes the breadcrumb state and renders the initial UI component.
/// It is intended to be the entry point for the main page content.
///
/// # Returns
///
/// * `Element` - The rendered home page element.
pub fn Home() -> Element {
    breadcrumbs!();

    rsx! {
        InitBox {}
    }
}