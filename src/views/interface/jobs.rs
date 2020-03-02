use crate::logger::curse_log;

use crate::state::StateManager;
use crate::state_retr;
use crate::views::interface::malwarehunter::MalwareHunterView;

use cursive::align::HAlign;

use cursive::views::{DummyView, LinearLayout, Panel, ResizedView, TextView};
use cursive::Cursive;

use std::sync::Mutex;

pub const JOB_TITLE_VIEW_NAME: &str = "job_title_view";
pub const WORKSPACE_VIEW_NAME: &str = "workspace_view";

#[derive(Clone)]
pub enum JobType {
    MalwareHunter,
}

pub struct Job {
    pub name: String,
    pub job_type: JobType,
}

pub fn complete_job(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    state_retr!(state_manager).remove_job();
    update_job_view(siv, state_manager);
}

pub fn update_job_view(siv: &mut Cursive, state_manager: &'static Mutex<StateManager>) {
    if state_retr!(state_manager).has_job() {
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
        let new_job = match state_retr!(state_manager).job.as_ref().unwrap().job_type  {
            JobType::MalwareHunter => MalwareHunterView::new(2, move |siv| complete_job(siv, state_manager))
        };
        siv.call_on_name(WORKSPACE_VIEW_NAME, |view: &mut LinearLayout| {
            view.remove_child(1);
            view.insert_child(
                1,
                ResizedView::with_full_screen(
                    Panel::new(new_job)
                        .title("Workspace")
                        .title_position(HAlign::Left),
                ),
            );
        });
    } else {
        siv.call_on_name(JOB_TITLE_VIEW_NAME, |view: &mut TextView| {
            view.set_content("");
        });
        siv.call_on_name(WORKSPACE_VIEW_NAME, |view: &mut LinearLayout| {
            view.remove_child(1);
            view.insert_child(
                1,
                ResizedView::with_full_screen(
                    Panel::new(DummyView)
                        .title("Workspace")
                        .title_position(HAlign::Left),
                ),
            );
        });
    }
}
