use nalgebra::Vector2;
use raylib::prelude::*;

use super::{Shoot, METEORS_SPEED};

#[derive(Copy, Clone, Default)]
pub enum MeteorKind {
    #[default]
    Big,
    Medium,
    Small,
}

impl MeteorKind {
    pub fn next(&self) -> Option<Self> {
        match self {
            Self::Big => Some(Self::Medium),
            Self::Medium => Some(Self::Small),
            Self::Small => None,
        }
    }

    pub fn get_radius(&self) -> f32 {
        match self {
            MeteorKind::Big => 40.0,
            MeteorKind::Medium => 20.0,
            MeteorKind::Small => 10.0,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Meteor {
    pub position: Vector2<f32>,
    pub speed: Vector2<f32>,
    pub radius: f32,
    pub active: bool,
    pub kind: MeteorKind,
    pub color: Color,
}

impl Meteor {
    pub fn update(&mut self, (width, height): (f32, f32)) {
        if self.active {
            self.position += self.speed;

            if self.position.x > width + self.radius {
                self.position.x = -self.radius;
            } else if self.position.x < -self.radius {
                self.position.x = width + self.radius;
            }

            if self.position.y > height + self.radius {
                self.position.y = -self.radius;
            } else if self.position.y < -self.radius {
                self.position.y = height + self.radius;
            }
        }
    }

    /// Split the meteor in two smaller parts, if possible.
    pub fn split(&self, shot: &Shoot) -> Option<[Self; 2]> {
        self.kind.next().map(|kind| {
            [-1.0, 1.0].map(|dir| Self {
                position: Vector2::new(self.position.x, self.position.y),
                speed: dir
                    * Vector2::new(
                        shot.rotation.to_radians().cos() * METEORS_SPEED * -1.0,
                        shot.rotation.to_radians().sin() * METEORS_SPEED * -1.0,
                    ),
                radius: kind.get_radius(),
                active: true,
                kind,
                color: Color::BLUE,
            })
        })
    }
}
