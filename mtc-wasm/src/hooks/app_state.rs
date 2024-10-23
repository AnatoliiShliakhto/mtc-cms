use super::*;

pub fn use_init_app_state() -> UseAppState {
    use_context_provider(|| UseAppState::default())
}

pub fn use_app_state() -> UseAppState {
    consume_context::<UseAppState>()
}

#[derive(Default, Clone, Copy)]
pub struct UseAppState {
    pub roles: Signal<Vec<Entry>>,
    pub groups: Signal<Vec<Entry>>,
}