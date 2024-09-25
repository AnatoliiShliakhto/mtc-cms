use super::*;
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(i32)]
pub enum SearchKind {
    LocalLink = 0,
    Link = 1,
    Course = 2,
    File = 100,
    FileWord = 101,
    FileExcel = 102,
    FilePowerPoint = 103,
    FilePdf = 104,
}