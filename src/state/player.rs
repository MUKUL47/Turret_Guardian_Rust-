use ggez::{event, glam::Vec2, GameResult};
use utils::{Pointf32};

#[path = "./gun_projectile.rs"]
mod gun_projectile;
#[path = "../utils/mod.rs"]
pub(crate) mod utils;
//
static TURRET_DISTANCE: f32 = 50.;

#[derive(Clone)]
pub struct Player {
    pub position: utils::Pointf32,
    pub angle: f32,
    pub gun_position: utils::Pointf32,
    win_size: Pointf32,
}
impl Player {
    pub fn new() -> Self {
        Player {
            position: (1., 1.),
            gun_position: (1., 1.),
            angle: (0.),
            win_size: (0., 0.),
        }
    }

    pub fn update(&mut self) -> &mut Self {
        self.angle = (self.angle + 0.025) % 360.;
        self.gun_position = utils::get_rotation_angle(
            &self.position,
            &self.gun_position,
            self.angle,
            Some(TURRET_DISTANCE),
        );
        return self;
    }

    pub fn update_position(&mut self, _x: f32, _y: f32) {
        self.position = (_x, _y);
    }
}
