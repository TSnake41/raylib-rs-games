use nalgebra::{Vector2, Vector3};
use raylib::{ffi::KeyboardKey, prelude::*};

use super::{PLAYER_SPEED, SHIP_HEIGHT};

#[derive(Default)]
pub struct Player {
    pub position: Vector2<f32>,
    pub speed: Vector2<f32>,
    pub acceleration: f32,
    pub rotation: f32,
    pub collider: Vector3<f32>,
    pub color: Color,
}

impl Player {
    pub fn process_player_movement(&mut self, rl: &RaylibHandle, screen_size: (f32, f32)) {
        let (width, height) = screen_size;

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.rotation -= 5f32;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.rotation += 5f32;
        }

        self.speed.x = self.rotation.to_radians().sin() * PLAYER_SPEED;
        self.speed.y = self.rotation.to_radians().cos() * PLAYER_SPEED;

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.acceleration = f32::min(self.acceleration + 0.04, 1.0);
        } else {
            self.acceleration = f32::max(0.0, self.acceleration - 0.03);
        }

        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.acceleration = f32::max(0.0, self.acceleration - 0.04);
        }

        self.position.x += self.speed.x * self.acceleration;
        self.position.y -= self.speed.y * self.acceleration;

        if self.position.x > width + SHIP_HEIGHT {
            self.position.x = -SHIP_HEIGHT;
        } else if self.position.x < -SHIP_HEIGHT {
            self.position.x = width + SHIP_HEIGHT;
        }

        if self.position.y > height + SHIP_HEIGHT {
            self.position.y = -SHIP_HEIGHT;
        } else if self.position.y < -SHIP_HEIGHT {
            self.position.y = height + SHIP_HEIGHT;
        }
    }
}
