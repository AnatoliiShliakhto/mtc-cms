use super::*;

pub fn use_init_personnel_columns() -> UsePersonnelColumns {
    let columns = UsePersonnelColumns {
        login: Signal::new(true),
        rank: Signal::new(true),
        name: Signal::new(true),
        ..Default::default()
    };
    use_context_provider(|| columns)
}

pub fn use_personnel_columns() -> UsePersonnelColumns {
    consume_context::<UsePersonnelColumns>()
}

#[derive(Default, Clone, Copy)]
pub struct UsePersonnelColumns {
    pub actions: Signal<bool>,
    pub login: Signal<bool>,
    pub rank: Signal<bool>,
    pub name: Signal<bool>,
    pub password: Signal<bool>,
    pub group: Signal<bool>,
    pub access: Signal<bool>,
}