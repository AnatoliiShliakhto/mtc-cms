use super::*;

pub fn use_init_busy() -> Signal<bool> {
    use_context_provider(UseBusy::default).inner
}

pub fn use_busy() -> Signal<bool> {
    consume_context::<UseBusy>().inner
}

#[derive(Default, Clone, Copy)]
pub struct UseBusy {
    inner: Signal<bool>,
}