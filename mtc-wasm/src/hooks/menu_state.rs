use super::*;

pub fn use_init_menu_state() -> Signal<bool> {
    use_context_provider(UseMenuState::default).inner
}

pub fn use_menu_state() -> Signal<bool> {
    consume_context::<UseMenuState>().inner
}

#[derive(Default, Clone, Copy)]
pub struct UseMenuState {
    inner: Signal<bool>,
}