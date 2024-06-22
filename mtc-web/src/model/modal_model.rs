#[derive(Clone, PartialEq)]
pub enum ModalModel {
    None,
    Alert(String),
    Info(String),
    Error(String),
    Success(String),
    Warning(String),
}