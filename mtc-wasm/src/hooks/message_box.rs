use super::*;

pub fn use_init_message_box() -> Signal<Option<MessageBoxArgs>> {
    use_context_provider(UseMessageBox::default).inner
}

pub fn use_message_box() -> Signal<Option<MessageBoxArgs>> {
    consume_context::<UseMessageBox>().inner
}

pub type MessageBoxArgs = (
    MessageKind,
    Cow<'static, str>,
    Option<MessageBoxFn>,
    Option<MessageBoxFnArgs>,
);

pub type MessageBoxFnArgs = (Cow<'static, str>, Option<Value>);
pub type MessageBoxFn = fn(Cow<'static, str>, Option<Value>);

#[derive(Default, Clone, Copy, PartialEq)]
pub struct UseMessageBox {
    inner: Signal<Option<MessageBoxArgs>>,
}