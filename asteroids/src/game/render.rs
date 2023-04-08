use std::cell::RefMut;

use super::*;
use raylib::core::text::measure_text;

impl Game {
    pub fn draw(&self, rl: &RaylibHandle, d: &RefMut<RaylibDrawHandle>) {
        let (width, height) = (rl.get_screen_width(), rl.get_screen_height());

        let half_width = width / 2;
        let half_height = height / 2;

        d.clear_background(Color::RAYWHITE);

        if !self.game_over {
            let cosf = f32::cos(self.player.rotation.to_radians());
            let sinf = f32::sin(self.player.rotation.to_radians());

            let tri = [
                self.player.position + Vector2::new(sinf * SHIP_HEIGHT, -cosf * SHIP_HEIGHT),
                self.player.position + Vector2::new(-cosf * 10f32, -sinf * 10f32),
                self.player.position + Vector2::new(cosf * 10f32, sinf * 10f32),
            ];

            d.draw_triangle(tri[0], tri[1], tri[2], self.player.color);

            for meteor in &self.meteors {
                d.draw_circle_v(
                    meteor.position,
                    meteor.radius,
                    if meteor.active {
                        meteor.color
                    } else {
                        Color::fade(&Color::LIGHTGRAY, 0.3)
                    },
                );
            }

            for shot in &self.shots {
                d.draw_circle_v(shot.position, shot.radius, shot.color);
            }

            if self.victory {
                d.draw_text(
                    "VICTORY",
                    half_width - measure_text("VICTORY", 20) / 2,
                    half_height,
                    20,
                    Color::LIGHTGRAY,
                );
            }

            if self.pause {
                d.draw_text(
                    "GAME PAUSED",
                    half_width - measure_text("GAME PAUSED", 40) / 2,
                    half_height - 40,
                    40,
                    Color::GRAY,
                );
            }
        } else {
            d.draw_text(
                "PRESS [ENTER] TO PLAY AGAIN",
                half_width - measure_text("PRESS [ENTER] TO PLAY AGAIN", 20) / 2,
                half_height - 50,
                20,
                Color::GRAY,
            );
        }
    }
}
