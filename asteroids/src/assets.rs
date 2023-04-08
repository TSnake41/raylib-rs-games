use raylib::prelude::*;

pub struct Assets<'bind, 'a> {
    pub explosion_sounds: Vec<Sound<'bind, 'a>>,
    pub shoot_sound: Option<Sound<'bind, 'a>>,
}

impl<'bind> Assets<'bind, '_> {
    pub fn load(raudio: &'bind RaylibAudio) -> Self {
        // Load as much explosionI.wav as available.
        let explosion_sounds = (1..)
            .map(|i| Sound::load_sound(raudio, format!("assets/explosion{i}.wav").as_str()).ok())
            .take_while(|r| r.is_some())
            .map(|o| o.unwrap())
            .collect();

        let shoot_sound = Sound::load_sound(raudio, "assets/laserShoot.wav").ok();

        Self {
            explosion_sounds,
            shoot_sound,
        }
    }

    pub fn play_explosion(&self, raudio: &RaylibAudio) {
        // Play a random explosion.
        if self.explosion_sounds.is_empty() {
            return;
        }

        let sound_index = fastrand::usize(0..self.explosion_sounds.len());
        raudio.play_sound(&self.explosion_sounds[sound_index]);
    }
}
