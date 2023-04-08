use nalgebra::Vector2;
use raylib::core::text::measure_text;
use raylib::ffi::{Color, Rectangle};
use raylib::prelude::*;

const PLAYER_MAX_LIFE: i32 = 5;
const LINES_OF_BRICKS: usize = 5;
const BRICKS_PER_LINE: usize = 20;

#[derive(Default)]
struct Player {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub life: i32,
}

#[derive(Default)]
struct Ball {
    position: Vector2<f32>,
    speed: Vector2<f32>,
    radius: i32,
    active: bool,
}

#[derive(Default)]
struct Brick {
    position: Vector2<f32>,
    color: Color,
}

struct Game {
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

fn main() {
    let (rl, thread) = raylib::init()
        .title("Arkanoid")
        .width(800)
        .height(480)
        .build();

    rl.set_target_fps(60);

    let (_w, _h) = (800, 480);

    let _game_over = false;
    let _pause = false;

    let mut game = Game::default();

    init_game(&mut game, &rl);

    while !rl.window_should_close() {
        update_game(&mut game, &rl);
        rl.begin_drawing(&thread, |d| draw_game(&game, &rl, &d));
    }
}

fn init_game(game: &mut Game, rl: &RaylibHandle) {
    let (w, h) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);
    game.brick_size = Vector2::new(rl.get_screen_width() as f32 / BRICKS_PER_LINE as f32, 40.0);

    // Initialize player
    game.player.position = Vector2::new(
        rl.get_screen_width() as f32 / 2.0,
        rl.get_screen_height() as f32 * 7.0 / 8.0,
    );
    game.player.size = Vector2::new(rl.get_screen_width() as f32 / 10.0, 20.0);
    game.player.life = PLAYER_MAX_LIFE;

    // Initialize ball
    game.ball.position = Vector2::new(w / 2.0, h * 7.0 / 7.0 - 30.0);
    game.ball.speed = Vector2::default();
    game.ball.radius = 7;
    game.ball.active = false;

    // Initialize bricks
    let initial_down_position = 50.0;

    game.bricks.clear();
    for i in 0..LINES_OF_BRICKS {
        for j in 0..BRICKS_PER_LINE {
            game.bricks.push(Brick {
                position: Vector2::new(
                    j as f32 * game.brick_size.x + game.brick_size.x / 2.0,
                    i as f32 * game.brick_size.y + initial_down_position,
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

fn update_game(game: &mut Game, rl: &RaylibHandle) {
    use raylib::ffi::KeyboardKey::*;
    let (w, h) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);

    if !game.game_over {
        if rl.is_key_pressed(KEY_P) {
            game.pause = !game.pause;
        }

        if !game.pause {
            // player movement logic
            if rl.is_key_down(KEY_LEFT) {
                game.player.position.x -= 5.0;
            }
            if game.player.position.x - game.player.size.x / 2.0 <= 0.0 {
                game.player.position.x = game.player.size.x / 2.0;
            }
            if rl.is_key_down(KEY_RIGHT) {
                game.player.position.x += 5.0;
            }
            if game.player.position.x + game.player.size.x / 2.0 >= w {
                game.player.position.x = w - game.player.size.x / 2.0;
            }

            // Ball launching logic
            if !game.ball.active {
                if rl.is_key_pressed(KEY_SPACE) {
                    game.ball.active = true;
                    game.ball.speed = Vector2::new(0.0, -5.0);
                }
            }

            // Ball movement logic
            if game.ball.active {
                game.ball.position += game.ball.speed;
            } else {
                game.ball.position = Vector2::new(game.player.position.x, h * 7.0 / 8.0 - 30.0);
            }

            // Collision logic: ball vs walls
            if game.ball.position.x + game.ball.radius as f32 >= w
                || game.ball.position.x - game.ball.radius as f32 <= 0.0
            {
                game.ball.speed.x *= -1.0;
            }
            if game.ball.position.y - game.ball.radius as f32 <= 0.0 {
                game.ball.speed.y *= -1.0;
            }
            if game.ball.position.y + game.ball.radius as f32 >= h {
                game.ball.speed = Vector2::default();
                game.ball.active = false;
                game.player.life -= 1;
            }

            // Collision logic: ball vs player
            let r = Rectangle::new(
                game.player.position.x - game.player.size.x / 2.0,
                game.player.position.y - game.player.size.y / 2.0,
                game.player.size.x,
                game.player.size.y,
            );
            if r.check_collision_circle_rec(game.ball.position.into(), game.ball.radius as f32) {
                if game.ball.speed.y > 0.0 {
                    game.ball.speed.y *= -1.0;
                    game.ball.speed.x = (game.ball.position.x - game.player.position.x)
                        / (game.player.size.x / 2.0)
                        * 5.0;
                }
            }

            // Collision logic: ball vs bricks
            game.bricks.retain_mut(|brick| {
                // Hit below
                if (game.ball.position.y - game.ball.radius as f32
                    <= brick.position.y + game.brick_size.y / 2.0)
                    && (game.ball.position.y - game.ball.radius as f32
                        > brick.position.y + game.brick_size.y / 2.0 + game.ball.speed.y)
                    && ((game.ball.position.x - brick.position.x).abs()
                        < game.brick_size.x / 2.0 + game.ball.radius as f32 * 2.0 / 3.0)
                    && game.ball.speed.y < 0.0
                {
                    game.ball.speed.y *= -1.0;
                    false
                }
                // Hit above
                else if game.ball.position.y + game.ball.radius as f32
                    >= brick.position.y - game.brick_size.y / 2.0
                    && (game.ball.position.y + game.ball.radius as f32)
                        .partial_cmp(
                            &(brick.position.y - game.brick_size.y / 2.0 + game.ball.speed.y),
                        )
                        .unwrap()
                        == std::cmp::Ordering::Less
                    && (game.ball.position.x - brick.position.x).abs()
                        < game.brick_size.x / 2.0 + game.ball.radius as f32 * 2.0 / 3.0
                    && game.ball.speed.y > 0.0
                {
                    game.ball.speed.y *= -1.0;
                    false
                }
                // Hit Left
                else if ((game.ball.position.x + game.ball.radius as f32)
                    >= (brick.position.x - game.brick_size.x / 2.0))
                    && ((game.ball.position.x + game.ball.radius as f32)
                        < (brick.position.x - game.brick_size.x / 2.0 + game.ball.speed.x))
                    && (((game.ball.position.y - brick.position.y).abs())
                        < (game.brick_size.y / 2.0 + game.ball.radius as f32 * 2.0 / 3.0))
                    && (game.ball.speed.x > 0.0)
                {
                    game.ball.speed.x *= -1.0;
                    false
                }
                // Hit right
                else if ((game.ball.position.x - game.ball.radius as f32)
                    <= (brick.position.x + game.brick_size.x / 2.0))
                    && ((game.ball.position.x - game.ball.radius as f32)
                        > (brick.position.x + game.brick_size.x / 2.0 + game.ball.speed.x))
                    && (((game.ball.position.y - brick.position.y).abs())
                        < (game.brick_size.y / 2.0 + game.ball.radius as f32 * 2.0 / 3.0))
                    && (game.ball.speed.x < 0.0)
                {
                    game.ball.speed.x *= -1.0;
                    false
                } else {
                    true
                }
            });

            // Game over condition
            if game.player.life <= 0 || game.bricks.is_empty() {
                game.game_over = true;
            }
        }
    } else {
        if rl.is_key_pressed(KEY_ENTER) {
            init_game(game, rl);
            game.game_over = false;
        }
    }
}

fn draw_game(game: &Game, rl: &RaylibHandle, d: &RaylibDrawHandle) {
    let (w, h) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);

    d.clear_background(Color::RAYWHITE);

    if !game.game_over {
        // Draw player bar
        d.draw_rectangle(
            (game.player.position.x - game.player.size.x / 2.0) as i32,
            (game.player.position.y - game.player.size.y / 2.0) as i32,
            game.player.size.x as i32,
            game.player.size.y as i32,
            Color::BLACK,
        );

        // Draw player lives
        for i in 0..game.player.life {
            d.draw_rectangle(20 + 30 * i, h as i32 - 30, 35, 10, Color::LIGHTGRAY);
        }

        // Draw ball
        d.draw_circle_v(game.ball.position, game.ball.radius as f32, Color::MAROON);

        // Draw bricks
        for brick in &game.bricks {
            d.draw_rectangle(
                (brick.position.x - game.brick_size.x / 2.0) as i32,
                (brick.position.y - game.brick_size.y / 2.0) as i32,
                game.brick_size.x as i32,
                game.brick_size.y as i32,
                brick.color,
            );
        }

        if game.pause {
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
