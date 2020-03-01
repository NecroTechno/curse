use crate::logger::curse_log;

use crate::state::StateManager;
use crate::state_retr;
use crate::views::interface::wordfinder::WordFinderView;

use cursive::align::HAlign;

use cursive::views::{LinearLayout, Panel, ResizedView, TextView};
use cursive::Cursive;

use std::sync::Mutex;

pub const JOB_TITLE_VIEW_NAME: &str = "job_title_view";
pub const WORKSPACE_VIEW_NAME: &str = "workspace_view";

pub enum JobType {
    WordFinder,
}

pub struct Job {
    pub name: String,
    pub job_type: JobType,
}

//pub fn complete_job(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
fn complete_job() {
    curse_log("Complete job");
}

pub fn update_job_view(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    siv.call_on_name(JOB_TITLE_VIEW_NAME, |view: &mut TextView| {
        view.set_content(
            state_retr!(state_manager)
                .job
                .as_ref()
                .unwrap()
                .name
                .clone(),
        );
    });
    let word_finder_job = WordFinderView::new(2, move |siv| complete_job());
    siv.call_on_name(WORKSPACE_VIEW_NAME, |view: &mut LinearLayout| {
        view.remove_child(1);
        view.insert_child(
            1,
            ResizedView::with_full_screen(
                Panel::new(word_finder_job)
                    .title("Workspace")
                    .title_position(HAlign::Left),
            ),
        );
    });
}
