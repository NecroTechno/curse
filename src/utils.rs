use crate::state::StateManager;
use crate::state_retr;
use crate::views;

use cursive::event::EventResult;
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::Cursive;

use std::sync::Mutex;

#[macro_export]
macro_rules! state_retr {
    ( $state_manager:ident ) => {
        $state_manager.lock().unwrap()
    };
}

pub fn view_open(
    siv: &mut Cursive,
    state_manager: &'static Mutex<StateManager>,
    view_category: &str,
) {
    siv.pop_layer();

    state_retr!(state_manager).update_view(view_category.to_string());
}

// TODO: Update with future view categories
pub fn load_saved_view(
    siv: &mut Cursive,
    state_manager: &'static Mutex<StateManager>,
) -> Option<String> {
    let view_to_load = state_manager.lock().unwrap().current_view.clone();
    match view_to_load.as_str() {
        "intro" => {
            views::intro::intro::intro_1(siv, state_manager);
            None
        }
        "interface" => {
            views::interface::interface::interface(siv, state_manager);
            None
        }
        _ => Some("View to be loaded not recognised.".to_string()),
    }
}

pub fn focus_se(state_manager: &'static Mutex<StateManager>) -> core::option::Option<EventResult> {
    state_manager
        .lock()
        .unwrap()
        .audio_manager
        .play_sound_effect("ui_accent.ogg");
    None
}

pub fn button_press_se(
    state_manager: &'static Mutex<StateManager>,
) -> core::option::Option<EventResult> {
    state_manager
        .lock()
        .unwrap()
        .audio_manager
        .play_sound_effect("ui_button_press.ogg");
    None
}

pub fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    // We'll return the current theme with a small modification.
    let mut theme = siv.current_theme().clone();

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;

    theme
}
