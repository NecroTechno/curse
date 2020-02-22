use crate::logger::curse_log;
use crate::state::StateManager;
use crate::state_retr;
use crate::utils::{button_press_se, focus_se};
use crate::views::jobs::{Job, JobType};

use cursive::event::{EventResult, EventTrigger};
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::views::{Button, Dialog, ListView, OnEventView, TextView};
use cursive::Cursive;

use std::sync::Mutex;
use std::sync::atomic::AtomicBool;

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
    fn accept_job(
        s: &mut Cursive,
        state_manager: &'static Mutex<StateManager>,
        notification_index: usize,
    ) {
        button_press_se(state_manager);
        if state_retr!(state_manager).has_job() {
            s.add_layer(
                Dialog::around(TextView::new("You already have a job in progress!"))
                .button("Ok", move |s| {
                    button_press_se(state_manager);
                    s.pop_layer();
                })
            );
        } else {
            let job_name = state_retr!(state_manager).notifications[notification_index]
                .title
                .clone();
            &state_retr!(state_manager).add_job(Job {
                name: job_name,
                job_type: JobType::WordFinder,
            });
            &state_retr!(state_manager)
                .notifications
                .remove(notification_index);
            s.call_on_name(NOTIFICATION_VIEW_NAME, |view: &mut ListView| {
                update_notifications(state_manager, view);
            });
            s.pop_layer();
        }
    }

    let popup = Dialog::around(TextView::new(notification.text_content.as_str()))
        .title(notification.title.as_str())
        .button("Accept Job", move |s| {
            accept_job(s, state_manager, notification_index)
        })
        .button("Close", move |s| {
            button_press_se(state_manager);
            s.pop_layer();
        });

    siv.add_layer(
        OnEventView::new(popup).on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
            focus_se(state_manager)
        }),
    )
}
