use nalgebra::Vector2;
use raylib::{core::text::measure_text, prelude::*};

use crate::assets::Assets;

const PLAYER_MAX_LIFE: i32 = 5;
const LINES_OF_BRICKS: usize = 5;
const BRICKS_PER_LINE: usize = 20;

#[derive(Default)]
pub struct Player {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub life: i32,
}

#[derive(Default)]
pub struct Ball {
    position: Vector2<f32>,
    speed: Vector2<f32>,
    radius: i32,
    active: bool,
}

#[derive(Default)]
pub struct Brick {
    position: Vector2<f32>,
    color: Color,
}

pub struct Game {
    game_over: bool,
    pause: bool,
    player: Player,
    ball: Ball,
    bricks: Vec<Brick>,
    brick_size: Vector2<f32>,
}

impl Default for Game {
    fn default() -> Game {
        let game_over = false;
        let pause = false;

        let player = Player::default();
        let ball = Ball::default();
        let bricks = Vec::new();
        let brick_size = Vector2::default();

        Game {
            game_over,
            pause,
            player,
            ball,
            brick_size,
            bricks,
        }
    }
}

impl Game {
    pub fn init(&mut self, rl: &RaylibHandle) {
        let (w, h) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        self.brick_size = Vector2::new(rl.get_screen_width() as f32 / BRICKS_PER_LINE as f32, 40.0);

        // Initialize player
        self.player.position = Vector2::new(
            rl.get_screen_width() as f32 / 2.0,
            rl.get_screen_height() as f32 * 7.0 / 8.0,
        );
        self.player.size = Vector2::new(rl.get_screen_width() as f32 / 10.0, 20.0);
        self.player.life = PLAYER_MAX_LIFE;

        // Initialize ball
        self.ball.position = Vector2::new(w / 2.0, h * 7.0 / 7.0 - 30.0);
        self.ball.speed = Vector2::default();
        self.ball.radius = 7;
        self.ball.active = false;

        // Initialize bricks
        let initial_down_position = 50.0;

        self.bricks.clear();
        for i in 0..LINES_OF_BRICKS {
            for j in 0..BRICKS_PER_LINE {
                self.bricks.push(Brick {
                    position: Vector2::new(
                        j as f32 * self.brick_size.x + self.brick_size.x / 2.0,
                        i as f32 * self.brick_size.y + initial_down_position,
                    ),
                    color: if (i + j) % 2 == 0 {
                        Color::GRAY
                    } else {
                        Color::LIGHTGRAY
                    },
                });
            }
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, raudio: &RaylibAudio, assets: &Assets) {
        use raylib::ffi::KeyboardKey::*;
        let (w, h) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        if !self.game_over {
            if rl.is_key_pressed(KEY_P) {
                self.pause = !self.pause;
            }

            if !self.pause {
                // player movement logic
                if rl.is_key_down(KEY_LEFT) {
                    self.player.position.x -= 5.0;
                }
                if self.player.position.x - self.player.size.x / 2.0 <= 0.0 {
                    self.player.position.x = self.player.size.x / 2.0;
                }
                if rl.is_key_down(KEY_RIGHT) {
                    self.player.position.x += 5.0;
                }
                if self.player.position.x + self.player.size.x / 2.0 >= w {
                    self.player.position.x = w - self.player.size.x / 2.0;
                }

                // Ball launching logic
                if !self.ball.active && rl.is_key_pressed(KEY_SPACE) {
                    self.ball.active = true;
                    self.ball.speed = Vector2::new(0.0, -5.0);
                }

                // Ball movement logic
                if self.ball.active {
                    self.ball.position += self.ball.speed;
                } else {
                    self.ball.position = Vector2::new(self.player.position.x, h * 7.0 / 8.0 - 30.0);
                }

                // Collision logic: ball vs walls
                if self.ball.position.x + self.ball.radius as f32 >= w
                    || self.ball.position.x - self.ball.radius as f32 <= 0.0
                {
                    self.ball.speed.x *= -1.0;
                    assets.play_bounce(raudio);
                }

                if self.ball.position.y - self.ball.radius as f32 <= 0.0 {
                    self.ball.speed.y *= -1.0;
                    assets.play_bounce(raudio);
                }

                if self.ball.position.y + self.ball.radius as f32 >= h {
                    self.ball.speed = Vector2::default();
                    self.ball.active = false;
                    self.player.life -= 1;
                }

                // Collision logic: ball vs player
                let r = Rectangle::new(
                    self.player.position.x - self.player.size.x / 2.0,
                    self.player.position.y - self.player.size.y / 2.0,
                    self.player.size.x,
                    self.player.size.y,
                );

                if r.check_collision_circle_rec(self.ball.position, self.ball.radius as f32)
                    && self.ball.speed.y > 0.0
                {
                    self.ball.speed.y *= -1.0;
                    self.ball.speed.x = (self.ball.position.x - self.player.position.x)
                        / (self.player.size.x / 2.0)
                        * 5.0;
                    assets.play_bounce(raudio);
                }

                // Collision logic: ball vs bricks
                self.bricks.retain_mut(|brick| {
                    // Hit below
                    if (self.ball.position.y - self.ball.radius as f32
                        <= brick.position.y + self.brick_size.y / 2.0)
                        && (self.ball.position.y - self.ball.radius as f32
                            > brick.position.y + self.brick_size.y / 2.0 + self.ball.speed.y)
                        && ((self.ball.position.x - brick.position.x).abs()
                            < self.brick_size.x / 2.0 + self.ball.radius as f32 * 2.0 / 3.0)
                        && self.ball.speed.y < 0.0
                    {
                        self.ball.speed.y *= -1.0;
                        assets.play_destroyed(raudio);
                        false
                    }
                    // Hit above
                    else if self.ball.position.y + self.ball.radius as f32
                        >= brick.position.y - self.brick_size.y / 2.0
                        && (self.ball.position.y + self.ball.radius as f32)
                            .partial_cmp(
                                &(brick.position.y - self.brick_size.y / 2.0 + self.ball.speed.y),
                            )
                            .unwrap()
                            == std::cmp::Ordering::Less
                        && (self.ball.position.x - brick.position.x).abs()
                            < self.brick_size.x / 2.0 + self.ball.radius as f32 * 2.0 / 3.0
                        && self.ball.speed.y > 0.0
                    {
                        self.ball.speed.y *= -1.0;
                        assets.play_bounce(raudio);
                        false
                    }
                    // Hit Left
                    else if ((self.ball.position.x + self.ball.radius as f32)
                        >= (brick.position.x - self.brick_size.x / 2.0))
                        && ((self.ball.position.x + self.ball.radius as f32)
                            < (brick.position.x - self.brick_size.x / 2.0 + self.ball.speed.x))
                        && (((self.ball.position.y - brick.position.y).abs())
                            < (self.brick_size.y / 2.0 + self.ball.radius as f32 * 2.0 / 3.0))
                        && (self.ball.speed.x > 0.0)
                    {
                        self.ball.speed.x *= -1.0;
                        assets.play_destroyed(raudio);
                        false
                    }
                    // Hit right
                    else if ((self.ball.position.x - self.ball.radius as f32)
                        <= (brick.position.x + self.brick_size.x / 2.0))
                        && ((self.ball.position.x - self.ball.radius as f32)
                            > (brick.position.x + self.brick_size.x / 2.0 + self.ball.speed.x))
                        && (((self.ball.position.y - brick.position.y).abs())
                            < (self.brick_size.y / 2.0 + self.ball.radius as f32 * 2.0 / 3.0))
                        && (self.ball.speed.x < 0.0)
                    {
                        self.ball.speed.x *= -1.0;
                        assets.play_destroyed(raudio);
                        false
                    } else {
                        true
                    }
                });

                // Game over condition
                if self.player.life <= 0 || self.bricks.is_empty() {
                    self.game_over = true;
                }
            }
        } else if rl.is_key_pressed(KEY_ENTER) {
            self.init(rl);
            self.game_over = false;
        }
    }

    pub fn draw(&self, rl: &RaylibHandle, d: &RaylibDrawHandle) {
        d.draw_fps(10, 10);

        let (w, h) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        d.clear_background(Color::RAYWHITE);

        if !self.game_over {
            // Draw player bar
            d.draw_rectangle(
                (self.player.position.x - self.player.size.x / 2.0) as i32,
                (self.player.position.y - self.player.size.y / 2.0) as i32,
                self.player.size.x as i32,
                self.player.size.y as i32,
                Color::BLACK,
            );

            // Draw player lives
            for i in 0..self.player.life {
                d.draw_rectangle(20 + 40 * i, h as i32 - 30, 35, 10, Color::LIGHTGRAY);
            }

            // Draw ball
            d.draw_circle_v(self.ball.position, self.ball.radius as f32, Color::MAROON);

            // Draw bricks
            for brick in &self.bricks {
                d.draw_rectangle(
                    (brick.position.x - self.brick_size.x / 2.0) as i32,
                    (brick.position.y - self.brick_size.y / 2.0) as i32,
                    self.brick_size.x as i32,
                    self.brick_size.y as i32,
                    brick.color,
                );
            }

            if self.pause {
                d.draw_text(
                    "Game Pause",
                    (w / 2.0) as i32 - measure_text("Game Paused", 40) / 2,
                    (h / 2.0 - 40.0) as i32,
                    40,
                    Color::GRAY,
                );
            }
        } else {
            d.draw_text(
                "PRESS [ENTER] TO PLAY AGAIN",
                (w / 2.0) as i32 - measure_text("PRESS [ENTER] TO PLAY AGAIN", 20) / 2,
                (h / 2.0) as i32 - 50,
                20,
                Color::GRAY,
            );
        }
    }
}
