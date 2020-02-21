use crate::logger::curse_log;
use crate::state::StateManager;
use crate::state_retr;
use crate::views;

use cursive::event::EventResult;
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::views::{Button, Dialog, ListView, TextView};
use cursive::Cursive;

use std::sync::Mutex;

pub struct Notification {
    pub text_content: String,
    pub title: String,
}

fn notification_dialog_builder(
    siv: &mut Cursive,
    state_manager: &'static Mutex<StateManager>,
    notification: &Notification,
) {
    siv.add_layer(
        Dialog::around(TextView::new(notification.text_content.as_str()))
            .title(notification.title.as_str())
            .dismiss_button("close"),
    );
}

pub fn check_for_notifications(
    state_manager: &'static Mutex<StateManager>,
    notifications_view: &mut ListView,
) {
    let notification_length = notifications_view.len();
    // TODO: decide if this needs to be updated all at once
    // it probably does lol
    if notifications_view.len() < state_retr!(state_manager).notifications.len() {
        notifications_view.add_child(
            format!(
                "Notif. {}",
                state_retr!(state_manager).notifications[notification_length].title
            )
            .as_str(),
            Button::new("Open", move |s| {
                notification_dialog_builder(
                    s,
                    state_manager,
                    &state_retr!(state_manager).notifications[notification_length],
                )
            }),
        );
    }
}
