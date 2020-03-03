use crate::state::StateManager;
use crate::state_retr;
use crate::utils::{focus_se, view_open};
use crate::vannah::animate;
use crate::views::common::logo_ani_generator;
use crate::views::interface::jobs::{JobType, JOB_TITLE_VIEW_NAME, WORKSPACE_VIEW_NAME};
use crate::views::interface::VIEW_CATEGORY;
use crate::views::notifications::{
    notification_content_generator, update_notifications, Notification, NOTIFICATION_VIEW_NAME,
};

use cursive::align::HAlign;
use cursive::event::{Event, EventTrigger};
use cursive::view::{Nameable, View};
use cursive::views::{
    Button, Canvas, Dialog, DummyView, EditView, LinearLayout, ListView, OnEventView, PaddedView,
    Panel, ResizedView, ScrollView, SelectView, TextView,
};
use cursive::Cursive;

use rand::Rng;

use std::sync::Mutex;

pub const MENU_VIEW_NAME: &str = "menu_view";

pub fn interface(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    view_open(siv, state_manager, VIEW_CATEGORY);

    let (animator_config, logo) = logo_ani_generator();

    let menu = LinearLayout::horizontal()
        .child(Button::new("Save.", move |_s| {
            state_retr!(state_manager).save();
        }))
        // temp
        .child(Button::new("Add notif", move |s| {
            let mut rng = rand::thread_rng();
            let notif_title: u8 = rng.gen();
            let text_content = notification_content_generator(state_manager);
            // TODO: convert impl?
            state_retr!(state_manager).notifications.push(Notification {
                text_content: text_content,
                title: format!("Job ID {}", notif_title),
                associated_job: JobType::MalwareHunter,
            });
            s.call_on_name(NOTIFICATION_VIEW_NAME, |view: &mut ListView| {
                update_notifications(state_manager, view);
            });
        }))
        .child(Button::new("Quit.", |s| s.quit()))
        .with_name(MENU_VIEW_NAME);

    let layout = PaddedView::lrtb(
        1,
        1,
        1,
        1,
        LinearLayout::horizontal()
            .child(
                LinearLayout::vertical()
                    .child(
                        Panel::new(ResizedView::with_min_height(
                            1,
                            TextView::empty().with_name(JOB_TITLE_VIEW_NAME),
                        ))
                        .title("Job Title")
                        .title_position(HAlign::Left),
                    )
                    .child(ResizedView::with_full_screen(
                        Panel::new(DummyView)
                            .title("Workspace")
                            .title_position(HAlign::Left),
                    ))
                    .with_name(WORKSPACE_VIEW_NAME),
            )
            .child(
                LinearLayout::vertical()
                    .child(PaddedView::lrtb(2, 2, 1, 1, logo))
                    .child(PaddedView::lrtb(
                        2,
                        2,
                        1,
                        2,
                        TextView::new(format!("Welcome, {}", state_retr!(state_manager).name))
                            .h_align(HAlign::Center),
                    ))
                    .child(Panel::new(PaddedView::lrtb(1, 1, 1, 1, menu)).title("Menu"))
                    .child(ResizedView::with_full_height(
                        // Notifications panel
                        Panel::new(ListView::new().with_name(NOTIFICATION_VIEW_NAME))
                            .title("Notifications"),
                    )),
            ),
    );

    siv.add_fullscreen_layer(
        OnEventView::new(layout)
            .on_event(Event::Refresh, move |s| {
                animate(&animator_config, s);
            })
            .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
                focus_se(state_manager)
            }),
    );
}
