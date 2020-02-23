use crate::state::StateManager;
use crate::state_retr;

use cursive::view::View;
use cursive::views::{TextView, SelectView};
use cursive::Cursive;

use std::sync::Mutex;

pub const JOB_TITLE_VIEW_NAME: &str = "job_title_view";
pub const WORKSPACE_VIEW_NAME: &str = "workspace_view";
pub const ENTRY_FIELD_VIEW_NAME: &str = "entry_field_view";

pub enum JobType {
    WordFinder,
}

pub struct Job {
    pub name: String,
    pub job_type: JobType,
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
    // siv.call_on_name(WORKSPACE_VIEW_NAME, |view: &mut SelectView| {
    //     view.add_item("ae", "ae".to_string());
    //     view.add_item("ae", "ae".to_string());
    //     view.add_item("ae", "ae".to_string());
    // });
}
