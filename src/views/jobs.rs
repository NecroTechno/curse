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
