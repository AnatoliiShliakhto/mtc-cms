use super::*;

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut dark_theme =
        use_local_storage("dark_theme", || false);
    
    let switch_theme = move |event: Event<FormData>| {
        dark_theme.set(!event.checked())
    };

    rsx! {
        div {
            class: "btn btn-ghost join-item",
            label { class: "swap swap-rotate",
                input {
                    value: "light",
                    r#type: "checkbox",
                    class: "theme-controller",
                    initial_checked: !dark_theme.get(),
                    onchange: switch_theme,
                }
                Icon { icon: Icons::Sun, class: "size-8 sm:size-6 fill-current swap-off" }
                Icon { icon: Icons::Moon, class: "size-8 sm:size-6 fill-current swap-on" }
            }
        }
    }
}