use nalgebra::{Vector2, Vector3};
use raylib::{core::collision::check_collision_circles, ffi::KeyboardKey, prelude::*};

use super::{meteor::MeteorKind, *};
use crate::assets::Assets;

impl Game {
    pub fn init(&mut self, rl: &RaylibHandle) {
        let (width, height) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        let half_width = width / 2.0;
        let half_height = height / 2.0;

        self.player.position = Vector2::new(half_width, half_height - (SHIP_HEIGHT / 2f32));
        self.player.acceleration = 0f32;
        self.player.collider = Vector3::new(
            self.player.position.x + self.player.rotation.to_radians().sin() * (SHIP_HEIGHT / 2.5),
            self.player.position.y - self.player.rotation.to_radians().cos() * (SHIP_HEIGHT / 2.5),
            12f32,
        );
        self.player.color = Color::MAROON;

        let mut correct_range = false;

        self.meteors.clear();
        for _ in 0..MAX_BIG_METEORS {
            let mut x: i32 = rl.get_random_value(0, width as i32);

            while !correct_range {
                if x > half_width as i32 - 150 && x < half_width as i32 + 150 {
                    x = rl.get_random_value(0, width as i32);
                } else {
                    correct_range = true;
                }
            }

            correct_range = false;

            let mut y: i32 = rl.get_random_value(0, height as i32);

            while !correct_range {
                if y > half_height as i32 - 150 && y < half_height as i32 + 150 {
                    y = rl.get_random_value(0, height as i32);
                } else {
                    correct_range = true;
                }
            }

            correct_range = false;

            let mut vel_x: i32 = rl.get_random_value(-METEORS_SPEED as i32, METEORS_SPEED as i32);
            let mut vel_y: i32 = rl.get_random_value(-METEORS_SPEED as i32, METEORS_SPEED as i32);

            while !correct_range {
                if vel_x == 0 && vel_y == 0 {
                    vel_x = rl.get_random_value(-METEORS_SPEED as i32, METEORS_SPEED as i32);
                    vel_y = rl.get_random_value(-METEORS_SPEED as i32, METEORS_SPEED as i32);
                } else {
                    correct_range = true;
                }
            }

            self.meteors.push(Meteor {
                position: Vector2::new(x as f32, y as f32),
                speed: Vector2::new(vel_x as f32, vel_y as f32),
                radius: MeteorKind::Big.get_radius(),
                active: true,
                color: Color::BLUE,
                kind: MeteorKind::Big,
            });
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, assets: &Assets, raudio: &RaylibAudio) {
        if !self.game_over {
            self.game_iteration(rl, assets, raudio);
        } else if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            self.init(rl);
            self.game_over = false;
        }
    }

    fn game_iteration(&mut self, rl: &RaylibHandle, assets: &Assets, raudio: &RaylibAudio) {
        let (width, height) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            self.pause = !self.pause;
        }

        if !self.pause {
            self.player.process_player_movement(rl, (width, height));

            if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                self.make_shot(assets, raudio);
            }

            self.process_shots(assets, raudio, (width, height));
            self.check_meteor_player_collisions();

            self.meteors
                .iter_mut()
                .for_each(|meteor| meteor.update((width, height)));
        }

        if self.meteors.iter().all(|meteor| !meteor.active) {
            self.victory = true;
        }
    }

    fn check_meteor_player_collisions(&mut self) {
        self.player.collider = Vector3::new(
            self.player.position.x + self.player.rotation.to_radians().sin() * (SHIP_HEIGHT / 2.5),
            self.player.position.y - self.player.rotation.to_radians().cos() * (SHIP_HEIGHT / 2.5),
            12f32,
        );

        if self.meteors.iter().any(|meteor| {
            meteor.active
                && check_collision_circles(
                    Vector2::new(self.player.collider.x, self.player.collider.y).into(),
                    self.player.collider.z,
                    meteor.position.into(),
                    meteor.radius,
                )
        }) {
            self.game_over = true;
        }
    }

    fn process_shots(
        &mut self,
        assets: &Assets,
        raudio: &RaylibAudio,
        (width, height): (f32, f32),
    ) {
        self.shots.retain_mut(|shot| {
            shot.life -= 1;

            shot.position.x += shot.speed.x;
            shot.position.y -= shot.speed.y;

            if (shot.position.x > width + shot.radius)
                || (shot.position.x < -shot.radius)
                || (shot.position.y > height + shot.radius)
                || (shot.position.y < -shot.radius)
                || (shot.life == 0)
            {
                return false;
            }

            if let Some(hit) = self.meteors.iter_mut().find(|meteor| {
                meteor.active
                    && check_collision_circles(
                        shot.position.into(),
                        shot.radius,
                        meteor.position.into(),
                        meteor.radius,
                    )
            }) {
                assets.play_explosion(raudio);

                hit.active = false;

                if let Some(splited) = hit.split(shot) {
                    self.meteors.extend(splited.iter());
                }

                return false;
            }

            true
        });
    }

    fn make_shot(&mut self, assets: &Assets, raudio: &RaylibAudio) {
        if let Some(ref shoot_sound) = assets.shoot_sound {
            raudio.play_sound(shoot_sound);
        }

        self.shots.push(Shoot {
            position: self.player.position
                + Vector2::new(
                    self.player.rotation.to_radians().sin() * SHIP_HEIGHT,
                    -self.player.rotation.to_radians().cos() * SHIP_HEIGHT,
                ),
            speed: 1.5
                * Vector2::new(
                    self.player.rotation.to_radians().sin() * PLAYER_SPEED,
                    self.player.rotation.to_radians().cos() * PLAYER_SPEED,
                ),
            rotation: self.player.rotation,
            radius: 2f32,
            life: 60,
            color: Color::BLACK,
        })
    }
}
