use crate::logger::curse_log;
use crate::state::StateManager;
use crate::state_retr;
use crate::utils::{button_press_se, focus_se};
use crate::views;

use cursive::event::{EventResult, EventTrigger};
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::views::{Button, Dialog, ListView, OnEventView, TextView};
use cursive::Cursive;

use std::sync::Mutex;

pub const NOTIFICATION_VIEW_NAME: &str = "notification_view";

pub struct Notification {
    pub text_content: String,
    pub title: String,
}

pub fn update_notifications(
    state_manager: &'static Mutex<StateManager>,
    notifications_view: &mut ListView,
) {
    if notifications_view.len() != state_retr!(state_manager).notifications.len() {
        notifications_view.clear();
        for (i, notification) in state_retr!(state_manager).notifications.iter().enumerate() {
            notifications_view.add_child(
                notification.title.as_str(),
                Button::new("Open", move |s| {
                    let mut notification_index: usize = 0;
                    s.call_on_name(NOTIFICATION_VIEW_NAME, |view: &mut ListView| {
                        notification_index = view.focus()
                    });
                    notification_dialog_builder(
                        s,
                        state_manager,
                        &state_retr!(state_manager).notifications[i],
                        notification_index,
                    )
                }),
            );
        }
    }
}

pub fn notification_content_generator(state_manager: &'static Mutex<StateManager>) -> String {
    format!("Hey {},\n\nI'm looking for someone to take care of this quick job for me. I'm swamped at the moment, can you do it for me?\n\nThanks!", &state_retr!(state_manager).name)
}

fn notification_dialog_builder(
    siv: &mut Cursive,
    state_manager: &'static Mutex<StateManager>,
    notification: &Notification,
    notification_index: usize,
) {
    siv.add_layer(
        OnEventView::new(
            Dialog::around(TextView::new(notification.text_content.as_str()))
                .title(notification.title.as_str())
                .button("Accept Job", move |s| {
                    button_press_se(state_manager);
                    &state_retr!(state_manager)
                        .notifications
                        .remove(notification_index);
                    s.call_on_name(NOTIFICATION_VIEW_NAME, |view: &mut ListView| {
                        update_notifications(state_manager, view);
                    });
                    s.pop_layer();
                })
                .button("Close", move |s| {
                    button_press_se(state_manager);
                    s.pop_layer();
                }),
        )
        .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
            focus_se(state_manager)
        }),
    )
}
