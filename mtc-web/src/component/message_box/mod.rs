#![allow(dead_code)]

use dioxus::prelude::*;

use crate::component::message_box::message_box_alert::MessageBoxAlert;
use crate::component::message_box::message_box_error::MessageBoxError;
use crate::component::message_box::message_box_info::MessageBoxInfo;
use crate::component::message_box::message_box_success::MessageBoxSuccess;
use crate::component::message_box::message_box_warning::MessageBoxWarning;

mod message_box_info;
mod message_box_error;
mod message_box_alert;
mod message_box_success;
mod message_box_warning;

#[derive(PartialEq, Props, Clone)]
pub struct MessageBoxComponentProps {
    pub kind: MessageBoxComponentKind,
    pub message: String,
}

#[derive(Clone, PartialEq)]
pub enum MessageBoxComponentKind {
    Alert,
    Info,
    Error,
    Success,
    Warning,
}

pub fn MessageBoxComponent(props: MessageBoxComponentProps) -> Element {
    match props.kind {
        MessageBoxComponentKind::Alert => MessageBoxAlert(props.message),
        MessageBoxComponentKind::Info => MessageBoxInfo(props.message),
        MessageBoxComponentKind::Error => MessageBoxError(props.message),
        MessageBoxComponentKind::Success => MessageBoxSuccess(props.message),
        MessageBoxComponentKind::Warning => MessageBoxWarning(props.message),
    }
}