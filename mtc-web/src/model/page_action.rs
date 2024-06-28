#[derive(Clone, PartialEq)]
pub enum PageAction {
    None,
    New,
    Selected(usize),
}