#[derive(Clone, PartialEq)]
pub enum PageAction {
    None,
    New,
    Item(String),
}