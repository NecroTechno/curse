//use crate::logger::curse_log;
use crate::state::StateManager;
use crate::state_retr;
use crate::utils::{button_press_se, focus_se, load_saved_view, view_open};
use crate::vannah::{Vannah, VannahConfig};
use crate::views::intro::intro::intro_1;

use cursive::align::HAlign;
use cursive::event::{Event, EventTrigger};
use cursive::view::Nameable;
use cursive::views::{Dialog, LinearLayout, OnEventView, SliderView, TextView};
use cursive::Cursive;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

const VIEW_CATEGORY: &str = "menu";

pub fn menu(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    view_open(siv, state_manager, VIEW_CATEGORY);

    let animator_config = VannahConfig {
        ani_ref: "title_ref",
        frames: vec![" ######  ##     ## ########   ######  ########\r\n##    ## ##     ## ##     ## ##    ## ##      \r\n##       ##     ## ##     ## ##       ##      \r\n##       ##     ## ########   ######  ######  \r\n##       ##     ## ##   ##         ## ##      \r\n##    ## ##     ## ##    ##  ##    ## ##      \r\n ######   #######  ##     ##  ######  ########", ".######..##.....##.########...######..########\r\n##....##.##.....##.##.....##.##....##.##......\r\n##.......##.....##.##.....##.##.......##......\r\n##.......##.....##.########...######..######..\r\n##.......##.....##.##...##.........##.##......\r\n##....##.##.....##.##....##..##....##.##......\r\n.######...#######..##.....##..######..########"],
        // Counter has to start at 1 to account for initial TextView frame
        vannah: Rc::new(RefCell::new(Vannah { counter: 1 })),
    };

    let title = TextView::new(animator_config.frames[0]).with_name(animator_config.ani_ref);
    let menu_layout = LinearLayout::vertical()
        .child(title)
        .child(TextView::new("\n A game by Eden").h_align(HAlign::Center));

    state_retr!(state_manager)
        .audio_manager
        .play_music_track("int_ody.ogg");

    siv.add_layer(
        OnEventView::new(
            Dialog::around(menu_layout)
                .title("Menu")
                .button("Begin", move |s| {
                    button_press_se(state_manager);
                    intro_1(s, state_manager);
                })
                .button("Load", move |s| {
                    button_press_se(state_manager);
                    let load_result = state_manager.lock().unwrap().load_save_state();
                    if let Some(mesg) = load_result {
                        load_error_popup(s, state_manager, &mesg);
                    } else {
                        let load_saved_view_option = load_saved_view(s, state_manager);
                        if let Some(mesg) = load_saved_view_option {
                            load_error_popup(s, state_manager, &mesg);
                        }
                    }
                })
                .button("Options", move |s| {
                    button_press_se(state_manager);
                    options_popup(s, state_manager);
                })
                .button("Quit", |s| s.quit()),
        )
        .on_event(Event::Refresh, move |s| {
            let frames = animator_config.frames.clone();
            animator_config
                .vannah
                .borrow_mut()
                .handle_animation(s, animator_config.ani_ref, frames)
        })
        .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
            focus_se(state_manager)
        }),
    );
}

fn load_error_popup(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>, message: &str) {
    let popup_inner = OnEventView::new(Dialog::around(TextView::new(message)).button(
        "Ok",
        move |s| {
            button_press_se(state_manager);
            s.pop_layer();
        },
    ))
    .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
        focus_se(state_manager)
    });

    siv.screen_mut().add_layer(popup_inner);
}

fn options_popup(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    let (initial_music_volume, initial_se_volume) =
        state_retr!(state_manager).audio_manager.get_track_volumes();

    let options_layout = LinearLayout::vertical()
        .child(TextView::new("Music Volume"))
        .child(
            LinearLayout::horizontal()
                .child(
                    SliderView::horizontal(101)
                        .value((initial_music_volume * 100_f32) as usize)
                        .on_change(move |s, v| {
                            let volume_value = format!("[ {}% ]", v);
                            s.call_on_name("music_track_volume", |view: &mut TextView| {
                                view.set_content(volume_value)
                            });
                            let update_value = v as f32 / 100_f32;
                            state_retr!(state_manager)
                                .audio_manager
                                .set_music_track_volume(update_value);
                        }),
                )
                .child(
                    TextView::new(format!("[ {}% ]", initial_music_volume * 100_f32))
                        .with_name("music_track_volume"),
                ),
        )
        .child(TextView::new("Sound Effects Volume"))
        .child(
            LinearLayout::horizontal()
                .child(
                    SliderView::horizontal(101)
                        .value((initial_se_volume * 100_f32) as usize)
                        .on_change(move |s, v| {
                            let volume_value = format!("[ {}% ]", v);
                            s.call_on_name("se_track_volume", |view: &mut TextView| {
                                view.set_content(volume_value)
                            });
                            let update_value = v as f32 / 100_f32;
                            state_retr!(state_manager)
                                .audio_manager
                                .set_se_track_volume(update_value);
                        }),
                )
                .child(
                    TextView::new(format!("[ {}% ]", initial_se_volume * 100_f32))
                        .with_name("se_track_volume"),
                ),
        );

    let option_layer = OnEventView::new(
        Dialog::around(options_layout)
            .button("Confirm", move |s| {
                button_press_se(state_manager);
                s.pop_layer();
            })
            .button("Cancel", move |s| {
                button_press_se(state_manager);
                state_retr!(state_manager)
                    .audio_manager
                    .set_music_track_volume(initial_music_volume);
                state_retr!(state_manager)
                    .audio_manager
                    .set_se_track_volume(initial_se_volume);
                s.pop_layer();
            }),
    )
    .on_pre_event_inner(EventTrigger::arrows(), move |_s, _e| {
        focus_se(state_manager)
    });

    siv.screen_mut().add_layer(option_layer);
}
