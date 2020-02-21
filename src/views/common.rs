use crate::vannah::{Vannah, VannahConfig};
use std::cell::RefCell;
use std::rc::Rc;

use cursive::view::Nameable;
use cursive::views::{NamedView, TextView};

fn logo_ani_frames() -> Vec<&'static str> {
    vec![" ######  ##     ## ########   ######  ########\r\n##    ## ##     ## ##     ## ##    ## ##      \r\n##       ##     ## ##     ## ##       ##      \r\n##       ##     ## ########   ######  ######  \r\n##       ##     ## ##   ##         ## ##      \r\n##    ## ##     ## ##    ##  ##    ## ##      \r\n ######   #######  ##     ##  ######  ########", ".######..##.....##.########...######..########\r\n##....##.##.....##.##.....##.##....##.##......\r\n##.......##.....##.##.....##.##.......##......\r\n##.......##.....##.########...######..######..\r\n##.......##.....##.##...##.........##.##......\r\n##....##.##.....##.##....##..##....##.##......\r\n.######...#######..##.....##..######..########"]
}

pub fn logo_ani_generator() -> (VannahConfig<'static>, NamedView<TextView>) {
    let animator_config = VannahConfig {
        ani_ref: "logo_ref",
        frames: logo_ani_frames(),
        // Counter has to start at 1 to account for initial TextView frame
        vannah: Rc::new(RefCell::new(Vannah { counter: 1 })),
    };

    let logo = TextView::new(animator_config.frames[0]).with_name(animator_config.ani_ref);

    (animator_config, logo)
}
