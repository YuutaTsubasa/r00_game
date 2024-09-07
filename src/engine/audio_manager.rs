use sdl2::mixer::Music;

pub struct AudioManager<'a> {
    current_music: Option<Music<'a>>,
    pub is_started: bool,
}

impl<'a> AudioManager<'a> {
    pub fn new(is_web: bool) -> Self {
        Self {
            current_music: None,
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

    pub fn start_music(&mut self){
        if self.is_started {
            return;
        }

        self.is_started = true;
        self.play_music();
    }
}