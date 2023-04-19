use raylib::prelude::*;

pub struct Assets<'bind, 'a> {
    pub destroyed_sounds: Vec<Sound<'bind, 'a>>,
    pub bounce_sound: Option<Sound<'bind, 'a>>,
}

impl<'bind> Assets<'bind, '_> {
    pub fn load(raudio: &'bind RaylibAudio) -> Self {
        // Load as much explosionI.wav as available.
        let destroyed_sounds = (1..)
            .map(|i| Sound::load_sound(raudio, format!("assets/explosion{i}.wav").as_str()).ok())
            .take_while(|r| r.is_some())
            .map(|o| o.unwrap())
            .collect();

        let bounce_sound = Sound::load_sound(raudio, "assets/bounce.wav").ok();

        Self {
            destroyed_sounds,
            bounce_sound,
        }
    }

    pub fn play_destroyed(&self, raudio: &RaylibAudio) {
        // Play a random explosion.
        if self.destroyed_sounds.is_empty() {
            return;
        }

        let sound_index = fastrand::usize(0..self.destroyed_sounds.len());
        raudio.play_sound(&self.destroyed_sounds[sound_index]);
    }

    pub fn play_bounce(&self, raudio: &RaylibAudio) {
        if let Some(bounce_sound) = &self.bounce_sound {
            raudio.play_sound(bounce_sound);
        }
    }
}
