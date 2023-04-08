use nalgebra::Vector2;
use raylib::prelude::*;

use self::{meteor::Meteor, player::Player};

mod logic;
mod meteor;
mod player;
mod render;

pub(crate) const SHIP_HEIGHT: f32 = 10f32 / 0.363970f32;
pub(crate) const PLAYER_SPEED: f32 = 6f32;
pub(crate) const MAX_BIG_METEORS: usize = 4;
pub(crate) const METEORS_SPEED: f32 = 2f32;
pub(crate) const MAX_SHOTS: usize = 10;

pub struct Game {
    game_over: bool,
    pause: bool,
    victory: bool,
    player: Player,
    meteors: Vec<Meteor>,
    shots: Vec<Shoot>,
}

#[derive(Default)]
pub struct Shoot {
    position: Vector2<f32>,
    speed: Vector2<f32>,
    radius: f32,
    rotation: f32,
    life: u8,
    color: Color,
}

impl Default for Game {
    fn default() -> Game {
        let game_over = false;
        let pause = false;
        let victory = false;

        let player = Player::default();
        let meteors = Vec::with_capacity(MAX_BIG_METEORS * 4);
        let shots = Vec::with_capacity(MAX_SHOTS);

        Game {
            game_over,
            pause,
            victory,
            player,
            meteors,
            shots,
        }
    }
}
