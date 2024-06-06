#![allow(dead_code)]

use dioxus::prelude::*;

use crate::widget::message_box::message_box_alert::MessageBoxAlert;
use crate::widget::message_box::message_box_error::MessageBoxError;
use crate::widget::message_box::message_box_info::MessageBoxInfo;
use crate::widget::message_box::message_box_success::MessageBoxSuccess;
use crate::widget::message_box::message_box_warning::MessageBoxWarning;

mod message_box_info;
mod message_box_error;
mod message_box_alert;
mod message_box_success;
mod message_box_warning;

#[derive(PartialEq, Props, Clone)]
pub struct MessageBoxWidgetProps {
    pub kind: MessageBoxWidgetKind,
    pub message: String,
}

#[derive(Clone, PartialEq)]
pub enum MessageBoxWidgetKind {
    Alert,
    Info,
    Error,
    Success,
    Warning,
}

pub fn MessageBoxWidget(props: MessageBoxWidgetProps) -> Element {
    match props.kind {
        MessageBoxWidgetKind::Alert => MessageBoxAlert(props.message),
        MessageBoxWidgetKind::Info => MessageBoxInfo(props.message),
        MessageBoxWidgetKind::Error => MessageBoxError(props.message),
        MessageBoxWidgetKind::Success => MessageBoxSuccess(props.message),
        MessageBoxWidgetKind::Warning => MessageBoxWarning(props.message),
    }
}