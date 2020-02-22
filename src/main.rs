extern crate log;
extern crate rodio;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod assets;
mod audio;
mod logger;
mod state;
mod utils;
mod vannah;
mod views;

use logger::curse_log;
use state::StateManager;
use views::menu::menu;
use views::resize::resize;

use cursive::Cursive;

use std::sync::Mutex;

lazy_static! {
    static ref STATE_MANAGER: Mutex<StateManager> = Mutex::new(StateManager::new());
}

fn main() {
    let mut siv = Cursive::default();

    let theme = utils::custom_theme_from_cursive(&siv);
    siv.set_theme(theme);

    siv.set_fps(2);

    #[cfg(debug_assertions)]
    siv.add_global_callback('~', Cursive::toggle_debug_console);

    curse_log("Run");

    let screen_size = siv.screen_size();

    if screen_size.x < 120 || screen_size.y < 30 {
        resize(&mut siv, &STATE_MANAGER)
    } else {
        // Run other views here to test immediately
        // interface(&mut siv, &STATE_MANAGER);
        menu(&mut siv, &STATE_MANAGER);
    }

    siv.run();
}
