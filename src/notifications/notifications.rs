use crate::logger::curse_log;
use crate::state::StateManager;
use crate::state_retr;
use crate::views;

use cursive::event::EventResult;
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::views::{Button, ListView};
use cursive::Cursive;

use std::sync::Mutex;

pub struct Notification {
    pub text_content: String,
    pub sender: String,
}

pub fn check_for_notifications(
    state_manager: &'static Mutex<StateManager>,
    notifications_view: &mut ListView,
) {
    let notification_length = notifications_view.len();
    if notifications_view.len() < state_retr!(state_manager).notifications.len() {
        notifications_view.add_child(
            format!(
                "Notif. {}",
                state_retr!(state_manager).notifications[notification_length].sender
            )
            .as_str(),
            Button::new("Open", |_s| ()),
        );
    }
}
