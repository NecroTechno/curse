use crate::assets::ASSETS_DIR;

use rodio::{Decoder, Sink, Source};

use std::io::Cursor;

pub struct AudioManager {
    music_track: Sink,
    se_track: Sink,
    music_volume: f32,
    se_volume: f32,
}

impl AudioManager {
    pub fn new() -> AudioManager {
        let music_device = rodio::default_output_device().unwrap();
        let se_device = rodio::default_output_device().unwrap();

        AudioManager {
            music_track: rodio::Sink::new(&music_device),
            se_track: rodio::Sink::new(&se_device),
            music_volume: 1.0,
            se_volume: 1.0,
        }
    }

    pub fn get_track_volumes(&mut self) -> (f32, f32) {
        (self.music_track.volume(), self.se_track.volume())
    }

    pub fn set_music_track_volume(&mut self, value: f32) {
        self.music_volume = value;
        self.music_track.set_volume(value);
    }

    pub fn set_se_track_volume(&mut self, value: f32) {
        self.se_volume = value;
        self.se_track.set_volume(value);
    }

    pub fn play_music_track(&mut self, file_name: &str) {
        if !self.music_track.empty() {
            self.music_track.stop();
            let music_device = rodio::default_output_device().unwrap();
            self.music_track = rodio::Sink::new(&music_device);
            self.music_track.set_volume(self.music_volume);
        }
        let file = Cursor::new(ASSETS_DIR.get_file(file_name).unwrap().contents());
        self.music_track
            .append(Decoder::new(file).unwrap().repeat_infinite());
        self.music_track.play();
    }

    pub fn _stop_music_track(&mut self) {
        self.music_track.stop();
        // Sinks cannot be reused after stopping. Below code replaces sink.
        // TODO: Set sink volume as stored in settings?
        let music_device = rodio::default_output_device().unwrap();
        self.music_track = rodio::Sink::new(&music_device);
        self.music_track.set_volume(self.music_volume);
    }

    pub fn play_sound_effect(&mut self, file_name: &str) {
        if !self.se_track.empty() {
            self.se_track.stop();
            let se_device = rodio::default_output_device().unwrap();
            self.se_track = rodio::Sink::new(&se_device);
            self.se_track.set_volume(self.se_volume);
        }
        let file = Cursor::new(ASSETS_DIR.get_file(file_name).unwrap().contents());
        self.se_track.append(Decoder::new(file).unwrap());
        self.se_track.play();
    }
}
