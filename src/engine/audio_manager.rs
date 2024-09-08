use sdl2::mixer::{Chunk, Music};

pub struct AudioManager<'a> {
    current_music: Option<Music<'a>>,
    current_sound: Option<Chunk>,
    pub is_started: bool,
}

impl<'a> AudioManager<'a> {
    pub fn new(is_web: bool) -> Self {
        Self {
            current_music: None,
            current_sound: None,
            is_started: !is_web
        }
    }

    pub fn load_music(&mut self, file_path: &str) {
        let music = Music::from_file(file_path).unwrap();
        self.current_music = Some(music);
    }

    pub fn play_music(&self) {
        if !self.is_started {
            return;
        }

        if let Some(music) = &self.current_music {
            music.play(-1).unwrap();
        }
    }

    pub fn play_sound_one_shot(&mut self, file_path: &str){
        let chunk = Chunk::from_file(file_path).unwrap();
        sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
        self.current_sound = Some(chunk);
    }

    pub fn start_music(&mut self){
        if self.is_started {
            return;
        }

        self.is_started = true;
        self.play_music();
    }
}