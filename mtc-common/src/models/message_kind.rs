/// Message kind enum
#[derive(Clone, PartialEq)]
pub enum MessageKind {
    Alert,
    Info,
    Error,
    Success,
    Warning,
}