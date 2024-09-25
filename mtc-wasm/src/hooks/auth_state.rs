use super::*;

pub fn use_init_auth_state() -> Signal<AuthState> {
    use_context_provider(|| Signal::new(AuthState::default()))
}

pub fn use_auth_state() -> Signal<AuthState> {
    use_context()
}