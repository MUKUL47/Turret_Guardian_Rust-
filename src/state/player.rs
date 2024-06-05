use ggez::{event, glam::Vec2, graphics::Color, GameResult};
use rand::Rng;
use utils::{get_rand_color, Pointf32};

#[path = "./gun_projectile.rs"]
mod gun_projectile;
#[path = "../utils/mod.rs"]
pub(crate) mod utils;
//
static TURRET_DISTANCE: f32 = 50.;

#[derive(Clone)]
pub struct Player {
    pub position: utils::Pointf32,
    pub turrets: [GunTurret; 4],
}
impl Player {
    pub fn new() -> Self {
        Player {
            position: (1., 1.),
            turrets: [
                GunTurret::new(true, TURRET_DISTANCE, 0.025),
                GunTurret::new(false, TURRET_DISTANCE + 2., 0.05),
                GunTurret::new(true, TURRET_DISTANCE + 3., 0.1),
                GunTurret::new(false, TURRET_DISTANCE + 4., 0.2),
            ],
        }
    }

    pub fn update(&mut self) -> &mut Self {
        for t in self.turrets.iter_mut() {
            t.update(
                if t.is_clockwise {
                    (t.angle + t.speed) % 360.
                } else {
                    if (t.angle - t.speed) <= 0. {
                        0.;
                    }
                    (t.angle - t.speed)
                },
                utils::get_rotation_angle(
                    &self.position,
                    &t.gun_position,
                    t.angle,
                    Some(t.distance_to_center),
                ),
            )
        }
        return self;
    }

    pub fn update_position(&mut self, _x: f32, _y: f32) {
        self.position = (_x, _y);
    }
}
#[derive(Clone)]
pub struct GunTurret {
    pub angle: f32,
    pub gun_position: utils::Pointf32,
    pub is_clockwise: bool,
    pub distance_to_center: f32,
    pub color: Color,
    pub speed: f32,
}
impl GunTurret {
    pub fn update(&mut self, angle: f32, gun_position: utils::Pointf32) {
        self.angle = angle;
        self.gun_position = gun_position;
    }
    pub fn new(is_clockwise: bool, distance_to_center: f32, speed: f32) -> Self {
        GunTurret {
            gun_position: (1., 1.),
            angle: rand::thread_rng().gen_range(0..=360) as f32,
            is_clockwise,
            distance_to_center,
            color: get_rand_color(),
            speed,
        }
    }
}
