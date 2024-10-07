use super::*;

pub fn use_init_pages_entries() -> Signal<Vec<Entry>> {
    use_context_provider(UsePagesEntries::default).inner
}

pub fn use_pages_entries() -> Signal<Vec<Entry>> {
    consume_context::<UsePagesEntries>().inner
}

#[derive(Default, Clone, Copy)]
pub struct UsePagesEntries {
    inner: Signal<Vec<Entry>>,
}