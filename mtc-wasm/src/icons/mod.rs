use crate::prelude::*;

mod menu;
mod sign_in;
mod lock;
mod error;
mod alert;
mod info;
mod success;
mod warning;
mod sun;
mod moon;
mod search;
mod close;
mod person;
mod settings;
mod shield_person;
mod sign_out;
mod home;
mod group;
mod info2;
mod book;
mod medal;
mod man;
mod work;
mod plus;
mod cancel;
mod refresh;
mod arrow_up;
mod link_45_deg;
mod link;
mod download;
mod file_word;
mod file_excel;
mod file_power_point;
mod file_pdf;
mod file;
mod diagram3;
mod back;
mod floppy;
mod trash;
mod ban;
mod user_check;
mod folder;
mod people;
mod description;
mod pen;
mod eye_slash;
mod eye;
mod database;
mod database_lock;
mod camera;
mod mortar_board;
mod journal_bookmark;
mod personnel;
mod copy;
mod paste;
mod upload;
mod incognito;
mod columns;
mod user_up;

pub mod prelude {
    pub use super::{
        Icon,
        Icons,
    };
}

#[derive(Clone, PartialEq)]
pub enum Icons {
    Menu,
    SignIn,
    Lock,
    Alert,
    Error,
    Info,
    Success,
    Warning,
    Sun,
    Moon,
    Search,
    Close,
    Person,
    Settings,
    ShieldPerson,
    SignOut,
    Home,
    Group,
    Info2,
    Book,
    Medal,
    Man,
    Work,
    Plus,
    Cancel,
    Refresh,
    ArrowUp,
    Link,
    Link45Deg,
    Download,
    FileExcel,
    FileWord,
    FilePowerPoint,
    FilePdf,
    File,
    Diagram3,
    Back,
    Floppy,
    Trash,
    Ban,
    UserCheck,
    Folder,
    People,
    Description,
    Pen,
    Eye,
    EyeSlash,
    Database,
    DatabaseLock,
    Camera,
    MortarBoard,
    JournalBookmark,
    Personnel,
    Copy,
    Paste,
    Upload,
    Incognito,
    Columns,
    UserUp,
}

#[component]
pub fn Icon(
    #[props]
    icon: Icons,
    #[props(into)]
    class: &'static str,
) -> Element {
    match icon {
        Icons::Menu => menu::MenuIcon(class),
        Icons::SignIn => sign_in::SignInIcon(class),
        Icons::Lock => lock::LockIcon(class),
        Icons::Alert => alert::AlertIcon(class),
        Icons::Error => error::ErrorIcon(class),
        Icons::Info => info::InfoIcon(class),
        Icons::Success => success::SuccessIcon(class),
        Icons::Warning => warning::WarningIcon(class),
        Icons::Sun => sun::SunIcon(class),
        Icons::Moon => moon::MoonIcon(class),
        Icons::Search => search::SearchIcon(class),
        Icons::Close => close::CloseIcon(class),
        Icons::Person => person::PersonIcon(class),
        Icons::Settings => settings::SettingsIcon(class),
        Icons::ShieldPerson => shield_person::ShieldPersonIcon(class),
        Icons::SignOut => sign_out::SignOutIcon(class),
        Icons::Home => home::HomeIcon(class),
        Icons::Group => group::GroupIcon(class),
        Icons::Info2 => info2::Info2Icon(class),
        Icons::Book => book::BookIcon(class),
        Icons::Medal => medal::MedalIcon(class),
        Icons::Man => man::ManIcon(class),
        Icons::Work => work::WorkIcon(class),
        Icons::Plus => plus::PlusIcon(class),
        Icons::Cancel => cancel::CancelIcon(class),
        Icons::Refresh => refresh::RefreshIcon(class),
        Icons::ArrowUp => arrow_up::ArrowUpIcon(class),
        Icons::Link45Deg => link_45_deg::Link45DegIcon(class),
        Icons::Link => link::LinkIcon(class),
        Icons::Download => download::DownloadIcon(class),
        Icons::FileExcel => file_excel::FileExcelIcon(class),
        Icons::FileWord => file_word::FileWordIcon(class),
        Icons::FilePowerPoint => file_power_point::FilePowerPointIcon(class),
        Icons::FilePdf => file_pdf::FilePdfIcon(class),
        Icons::File => file::FileIcon(class),
        Icons::Diagram3 => diagram3::Diagram3Icon(class),
        Icons::Back => back::BackIcon(class),
        Icons::Floppy => floppy::FloppyIcon(class),
        Icons::Trash => trash::TrashIcon(class),
        Icons::Ban => ban::BanIcon(class),
        Icons::UserCheck => user_check::UserCheckIcon(class),
        Icons::Folder => folder::FolderIcon(class),
        Icons::People => people::PeopleIcon(class),
        Icons::Description => description::DescriptionIcon(class),
        Icons::Pen => pen::PenIcon(class),
        Icons::Eye => eye::EyeIcon(class),
        Icons::EyeSlash => eye_slash::EyeSlashIcon(class),
        Icons::Database => database::DatabaseIcon(class),
        Icons::DatabaseLock => database_lock::DatabaseLockIcon(class),
        Icons::Camera => camera::CameraIcon(class),
        Icons::MortarBoard  => mortar_board::MortarBoardIcon(class),
        Icons::JournalBookmark  => journal_bookmark::JournalBookmarkIcon(class),
        Icons::Personnel => personnel::PersonnelIcon(class),
        Icons::Copy => copy::CopyIcon(class),
        Icons::Paste => paste::PasteIcon(class),
        Icons::Upload => upload::UploadIcon(class),
        Icons::Incognito => incognito::IncognitoIcon(class),
        Icons::Columns => columns::ColumnsIcon(class),
        Icons::UserUp => user_up::UserUpIcon(class),
    }
}