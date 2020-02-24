//use crate::logger::curse_log;
use crate::audio::AudioManager;
use crate::views::interface::jobs::Job;
use crate::views::notifications::Notification;

use serde::{Deserialize, Serialize};
use serde_json::to_writer;

use std::fs::File;
use std::io::Read;

// TODO: convert saved/current view to enum

#[derive(Serialize, Deserialize)]
struct SaveState {
    pub saved_view: String,
    pub name: String,
}

pub struct StateManager {
    pub audio_manager: AudioManager,
    pub current_view: String,
    pub name: String,
    pub notifications: Vec<Notification>,
    pub job: Option<Job>,
}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            audio_manager: AudioManager::new(),
            current_view: "".to_string(),
            name: "".to_string(),
            notifications: Vec::<Notification>::new(),
            job: None,
        }
    }

    pub fn update_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn update_view(&mut self, view: String) {
        self.current_view = view;
    }

    pub fn save(&self) {
        let save_state = SaveState {
            saved_view: self.current_view.clone(),
            name: self.name.clone(),
        };
        to_writer(&File::create("save_data.json").unwrap(), &save_state).unwrap();
    }

    pub fn add_job(&mut self, new_job: Job) {
        self.job = Some(new_job);
    }

    pub fn has_job(&self) -> bool {
        self.job.is_some()
    }

    pub fn load_save_state(&mut self) -> Option<String> {
        let save_file = File::open("save_data.json");
        match save_file {
            Ok(mut save_file) => {
                let mut buff = String::new();
                save_file.read_to_string(&mut buff).unwrap();

                let load_state: std::result::Result<SaveState, serde_json::Error> =
                    serde_json::from_str(&buff);

                match load_state {
                    Ok(load_state) => {
                        self.current_view = load_state.saved_view;
                        self.name = load_state.name;

                        return None;
                    }
                    Err(_error) => return Some("Save data malformed.".to_string()),
                }
            }
            Err(_error) => return Some("Failed to load.".to_string()),
        };
    }
}
