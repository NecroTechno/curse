use cursive::views::TextView;
use cursive::Cursive;

use std::cell::RefCell;
use std::rc::Rc;

// The initial config for a TextView animation
pub struct VannahConfig<'a> {
    // The textview name
    pub ani_ref: &'a str,
    // The frames to be animated
    pub frames: Vec<&'a str>,
    // The animator struct
    pub vannah: Rc<RefCell<Vannah>>,
}

#[derive(Copy, Clone)]
pub struct Vannah {
    pub counter: usize,
}

impl Vannah {
    pub fn handle_animation(&mut self, siv: &mut Cursive, ani_ref: &str, frames: Vec<&str>) {
        let new_content: &str = frames[self.counter];

        if self.counter == frames.len() - 1 {
            self.counter = 0;
        } else {
            self.counter += 1;
        }

        siv.call_on_name(ani_ref, |view: &mut TextView| {
            view.set_content(new_content);
        });
    }
}

pub fn animate(animator_config: &VannahConfig, siv: &mut Cursive) {
    let frames = animator_config.frames.clone();
    animator_config
        .vannah
        .borrow_mut()
        .handle_animation(siv, animator_config.ani_ref, frames)
}
