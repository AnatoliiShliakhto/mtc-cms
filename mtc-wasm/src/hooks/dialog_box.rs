use super::*;

pub fn use_init_dialog_box() -> Signal<Option<DialogBoxArgs>> {
    use_context_provider(UseDialogBox::default).inner
}

pub fn use_dialog_box() -> Signal<Option<DialogBoxArgs>> {
    consume_context::<UseDialogBox>().inner
}

#[derive(Clone, PartialEq)]
pub struct  DialogBoxArgs {
    pub kind: MessageKind,
    pub message: Cow<'static, str>,
    pub handler: Option<EventHandler<MouseEvent>>,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct UseDialogBox {
    inner: Signal<Option<DialogBoxArgs>>,
}