//use crate::logger::curse_log;
use crate::state::StateManager;
use crate::utils::{button_press_se, focus_se, view_open};
use crate::state_retr;

use cursive::views::{Dialog, OnEventView, TextView};
use cursive::Cursive;
use cursive::event::{Event, EventTrigger};

use std::sync::Mutex;

// Can be accessed in mod file
const VIEW_CATEGORY: &str = "TEMPLATE";

pub fn intro_1(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    siv.pop_layer();

    siv.add_layer(
        OnEventView::new(
            Dialog::around(TextView::new("Test"))
                .title("Hello"),
        )
        .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
            focus_se(state_manager)
        }),
    );
}
