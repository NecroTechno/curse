//use crate::logger::curse_log;
use crate::state::StateManager;
use crate::state_retr;
use crate::utils::{button_press_se, focus_se, view_open};
use crate::views::intro::VIEW_CATEGORY;

use crate::views::interface::interface::interface;
use crate::views::menu::menu;

use cursive::event::EventTrigger;
use cursive::view::Nameable;
use cursive::views::{Dialog, EditView, OnEventView, ResizedView, ScrollView, TextView};
use cursive::Cursive;

use std::sync::Mutex;

pub fn intro_1(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    view_open(siv, state_manager, VIEW_CATEGORY);

    let terms = TextView::new(
        "Your privacy is *extremely* important to us.\r\n\r\nNot quite as important as us collecting all your delectable data. But still, pretty important. On a scale of one to ten, your privacy ranks a very respectable five.\r\n\r\nYou can trust that - after we've analysed and processed and siphoned every last bit of commercial value out of your data - the useless residue overflowing our data lakes will be treated in full compliance with the best privacy legislation lobbyist money can buy.\r\n\r\nAnd that's a promise.\r\n\r\nBrian Dugnutt - Chief Data Officer at SHADOW."
    );

    siv.add_layer(ResizedView::with_max_size(
        (120, 30),
        OnEventView::new(
            Dialog::around(ScrollView::new(terms))
                .title("Privacy Policy")
                .button("Sounds dope", move |s| {
                    button_press_se(state_manager);
                    intro_2(s, state_manager)
                })
                .button("This sucks", move |s| {
                    button_press_se(state_manager);
                    reject_popup(s, state_manager);
                }),
        )
        .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
            focus_se(state_manager)
        }),
    ));
}

fn intro_2(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    view_open(siv, state_manager, VIEW_CATEGORY);

    let text = TextView::new(
        "Hello!\r\n\r\nYou have been successful in the recent round of applications for the role of OPERATOR at SHADOW. Congratulations!\r\n\r\nIn years past, OPERATORS were full time, on-site hires. However, at SHADOW, we take pride in innovation and worker satisfaction; we realised our employees didn't want to endure long commutes to labour away in a cramped office. That's why all our OPERATORS have been transitioned to completely independent, remote contractors. We want you to have independence and freedom in your interactions with our company - as such, we see our OPERATORS less like employees and more like partners.\r\n\r\nThis is how you can conduct your work right from the comfort of your own home!"
    );

    siv.add_layer(ResizedView::with_max_size(
        (120, 30),
        OnEventView::new(
            Dialog::around(ScrollView::new(text))
                .title("Introduction")
                .button("Next", move |s| {
                    button_press_se(state_manager);
                    intro_3(s, state_manager)
                }),
        )
        .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
            focus_se(state_manager)
        }),
    ));
}

fn intro_3(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    view_open(siv, state_manager, VIEW_CATEGORY);

    let text = TextView::new(
        "First things first: let's get you set up in the system. On the following page, please enter your name."
    );

    siv.add_layer(ResizedView::with_max_size(
        (120, 30),
        OnEventView::new(
            Dialog::around(ScrollView::new(text))
                .title("Introduction")
                .button("Next", move |s| {
                    button_press_se(state_manager);
                    intro_4(s, state_manager)
                }),
        )
        .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
            focus_se(state_manager)
        }),
    ));
}

fn intro_4(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    view_open(siv, state_manager, VIEW_CATEGORY);

    siv.add_layer(ResizedView::with_max_size(
        (120, 30),
        OnEventView::new(
            Dialog::around(EditView::new().with_name("name_field"))
                .title("Enter Your Name")
                .button("Submit", move |s| {
                    let name = s
                        .call_on_name("name_field", |view: &mut EditView| view.get_content())
                        .unwrap();
                    button_press_se(state_manager);
                    state_retr!(state_manager).update_name(name.to_string());
                    intro_5(s, state_manager);
                }),
        )
        .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
            focus_se(state_manager)
        }),
    ));
}

fn intro_5(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    view_open(siv, state_manager, VIEW_CATEGORY);

    let text = TextView::new(
        format!("Welcome {}.\r\n\r\nSoon you will be introduced to the CURSE interface. It might be a little overwhelming at first, but don't worry, we trust in a short amount of time you will feel comfortable, and most importantly, productive while using it.", &state_retr!(state_manager).name)
    );

    siv.add_layer(ResizedView::with_max_size(
        (120, 30),
        OnEventView::new(
            Dialog::around(ScrollView::new(text))
                .title("Introduction")
                .button("Next", move |s| {
                    button_press_se(state_manager);
                    interface(s, state_manager);
                }),
        )
        .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
            focus_se(state_manager)
        }),
    ));
}

fn reject_popup(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    siv.add_layer(
        Dialog::around(TextView::new("Love it or leave it, baby."))
            .title("Fuck You")
            .button("hmmm...", move |s| {
                button_press_se(state_manager);
                s.pop_layer();
            }),
    )
}
