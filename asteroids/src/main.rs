use assets::Assets;
use game::Game;

use raylib::audio::RaylibAudio;

mod assets;
mod game;

fn main() {
    let (rl, thread) = raylib::init()
        .title("Asteroids")
        .width(800)
        .height(480)
        .build();
    let raudio = RaylibAudio::init_audio_device();

    rl.set_target_fps(60);

    let mut game = Game::default();
    let assets = Assets::load(&raudio);

    game.init(&rl);

    while !rl.window_should_close() {
        game.update(&rl, &assets, &raudio);
        rl.begin_drawing(&thread, |d| game.draw(&rl, &d))
    }
}
