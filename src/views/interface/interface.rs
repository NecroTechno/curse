//use crate::logger::curse_log;
use crate::state::StateManager;
use crate::state_retr;
use crate::utils::{button_press_se, focus_se, view_open};
use crate::vannah::{Vannah, VannahConfig};
use crate::views::interface::VIEW_CATEGORY;

use crate::views::menu::menu;

use cursive::align::HAlign;
use cursive::event::{Event, EventTrigger};
use cursive::view::Nameable;
use cursive::views::{
    Dialog, EditView, LinearLayout, OnEventView, PaddedView, Panel, ResizedView, ScrollView,
    TextView,
};
use cursive::Cursive;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

pub fn interface(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    view_open(siv, state_manager, VIEW_CATEGORY);

    let animator_config = VannahConfig {
        ani_ref: "logo_ref",
        frames: vec![" ######  ##     ## ########   ######  ########\r\n##    ## ##     ## ##     ## ##    ## ##      \r\n##       ##     ## ##     ## ##       ##      \r\n##       ##     ## ########   ######  ######  \r\n##       ##     ## ##   ##         ## ##      \r\n##    ## ##     ## ##    ##  ##    ## ##      \r\n ######   #######  ##     ##  ######  ########", ".######..##.....##.########...######..########\r\n##....##.##.....##.##.....##.##....##.##......\r\n##.......##.....##.##.....##.##.......##......\r\n##.......##.....##.########...######..######..\r\n##.......##.....##.##...##.........##.##......\r\n##....##.##.....##.##....##..##....##.##......\r\n.######...#######..##.....##..######..########"],
        // Counter has to start at 1 to account for initial TextView frame
        vannah: Rc::new(RefCell::new(Vannah { counter: 1 })),
    };

    let logo = TextView::new(animator_config.frames[0])
        .h_align(HAlign::Center)
        .with_name(animator_config.ani_ref);

    let layout = PaddedView::lrtb(
        1,
        1,
        1,
        1,
        LinearLayout::horizontal()
            .child(ResizedView::with_full_screen(
                Panel::new(TextView::new("Test.")).title("Title 1"),
            ))
            .child(
                LinearLayout::vertical()
                    .child(PaddedView::lrtb(2, 2, 1, 2, logo))
                    .child(ResizedView::with_full_height(
                        // Notifications panel
                        Panel::new(
                            LinearLayout::vertical()
                                .child(TextView::new("Test."))
                                .child(TextView::new("Test.")),
                        )
                        .title("Notifications"),
                    )),
            ),
    );

    siv.add_fullscreen_layer(
        OnEventView::new(layout)
            .on_event(Event::Refresh, move |s| {
                let frames = animator_config.frames.clone();
                animator_config.vannah.borrow_mut().handle_animation(
                    s,
                    animator_config.ani_ref,
                    frames,
                )
            })
            .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
                focus_se(state_manager)
            }),
    );
}
