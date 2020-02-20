//use crate::logger::curse_log;
use crate::state::StateManager;
use crate::state_retr;
use crate::utils::{button_press_se, focus_se, view_open};
use crate::vannah::{Vannah, VannahConfig, animate};
use crate::views::common::logo_ani_generator;
use crate::views::interface::VIEW_CATEGORY;

use crate::views::menu::menu;

use cursive::align::HAlign;
use cursive::event::{Event, EventTrigger};
use cursive::view::Nameable;
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, OnEventView, PaddedView, Panel, ResizedView,
    ScrollView, TextView, Canvas, ListView
};
use cursive::Cursive;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

pub fn interface(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    view_open(siv, state_manager, VIEW_CATEGORY);

    let (animator_config, logo) = logo_ani_generator();

    let menu = LinearLayout::horizontal()
        .child(Button::new("Save.", move |_s| {
            state_retr!(state_manager).save();
        }))
        .child(Button::new("Quit.", |s| s.quit()));

    let state = String::new();
    let canvas = Canvas::new(state);

    let layout = PaddedView::lrtb(
        1,
        1,
        1,
        1,
        LinearLayout::horizontal()
            .child(ResizedView::with_full_screen(
                Panel::new(canvas).title("Title 1"),
            ))
            .child(
                LinearLayout::vertical()
                    .child(PaddedView::lrtb(2, 2, 1, 2, logo))
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
                        Panel::new(
                            ListView::new()
                                //.child(TextView::new("Test."))
                                //.child(TextView::new("Test.")),
                        )
                        .title("Notifications"),
                    )),
            ),
    );

    siv.add_fullscreen_layer(
        OnEventView::new(layout)
            .on_event(Event::Refresh, move |s| {
                animate(&animator_config, s)
            })
            .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
                focus_se(state_manager)
            }),
    );
}
