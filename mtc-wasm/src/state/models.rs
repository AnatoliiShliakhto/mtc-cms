use super::*;

#[derive(Clone, PartialEq)]
pub struct  DialogBoxArgs {
    pub kind: MessageKind,
    pub message: Cow<'static, str>,
    pub handler: Option<EventHandler<MouseEvent>>,
}

#[derive(Deserialize, Clone)]
pub struct I18nEntry {
    pub key: Cow<'static, str>,
    pub value: Cow<'static, str>,
}

#[derive(Clone, Copy)]
pub struct PersonnelColumns {
    pub actions: Signal<bool>,
    pub login: Signal<bool>,
    pub rank: Signal<bool>,
    pub name: Signal<bool>,
    pub password: Signal<bool>,
    pub group: Signal<bool>,
    pub access: Signal<bool>,
}

impl Default for PersonnelColumns {
    fn default() -> Self {
        Self {
            actions: Default::default(),
            login: Signal::new(true),
            rank: Signal::new(true),
            name: Signal::new(true),
            password: Default::default(),
            group: Default::default(),
            access: Default::default(),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct SearchEngine {
    pub list: Signal<BTreeMap<usize, SearchIdxDto>>,
    pub index: Signal<simsearch::SimSearch<usize>>,
    pub result: Signal<Vec<SearchIdxDto>>,
    pub pattern: Signal<Cow<'static, str>>,
}