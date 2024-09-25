use super::*;

pub fn close_message_box_task(event: Event<MouseData>) {
    *use_message_box().write_unchecked() = None
}