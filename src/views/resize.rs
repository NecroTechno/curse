//use crate::logger::curse_log;
use crate::state::StateManager;
use crate::views::menu::menu;

use cursive::event::Event;
use cursive::views::{Dialog, OnEventView, TextView};
use cursive::Cursive;

use std::sync::Mutex;

pub fn resize(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    siv.pop_layer();

    siv.add_layer(
        OnEventView::new(
            Dialog::around(TextView::new("Please increase the size of your terminal."))
                .title("Menu"),
        )
        .on_event(Event::WindowResize, move |s| {
            let screen_size = s.screen_size();
            if screen_size.x >= 120 && screen_size.y >= 30 {
                menu(s, state_manager);
            }
        }),
    );
}
