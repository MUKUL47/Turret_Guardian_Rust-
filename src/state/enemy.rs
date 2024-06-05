use rand::*;
use utils::Pointf32;

use super::gun_projectile;

#[path = "../utils/mod.rs"]
pub(crate) mod utils;

#[derive(Clone)]
pub struct Enemy {
    pub position: utils::Pointf32,
    distance_from_target: f32,
    pub size: f32,
}
#[derive(Clone)]

pub struct Enemies {
    pub enemies: Vec<Enemy>,
    pub win_size: Pointf32,
    wave: i32,
    next_wave: bool,
    update_counter: i8,
}

impl Enemies {
    pub fn new() -> Self {
        Enemies {
            enemies: vec![],
            win_size: (0., 0.),
            wave: 0,
            update_counter: 0,
            next_wave: false,
        }
    }

    pub fn check_projectile_collision(&mut self, gun_projectile: &mut gun_projectile::Projectiles) {
        for projectile in gun_projectile.projectiles.iter_mut() {
            if projectile.is_deleted {
                continue;
            }
            self.enemies.retain_mut(|e| {
                let is_collided = (f32::abs(projectile.position.0 - e.position.0) < (e.size + 2.5)
                    && f32::abs(projectile.position.1 - e.position.1) < (e.size + 2.5));
                if is_collided {
                    projectile.mark_deleted();
                }
                return !is_collided;
            });
        }
        self.next_wave = self.enemies.len() == 0;
    }

    fn update_enemies(&mut self) {
        self.update_counter = self.update_counter.wrapping_add(1);
        if self.update_counter % 100 != 0 {
            return;
        }
        let mut rng = rand::thread_rng();
        let direction_rand: i32 = rng.gen_range(1..=4);
        let origin: (f32, f32) = if direction_rand == 1 {
            (0., rng.gen_range(0..=self.win_size.1 as i32) as f32)
        } else if direction_rand == 2 {
            (rng.gen_range(0..=self.win_size.0 as i32) as f32, 0.)
        } else if direction_rand == 3 {
            (
                rng.gen_range(0..=self.win_size.0 as i32) as f32,
                self.win_size.1,
            )
        } else {
            (
                self.win_size.0,
                rng.gen_range(0..=self.win_size.1 as i32) as f32,
            )
        };
        self.enemies.push(Enemy {
            distance_from_target: utils::euclidean_distance(&(0., 0.), &origin),
            position: origin,
            size: rand::thread_rng().gen_range(10..=20) as f32,
        });
    }

    pub fn update(&mut self, target: &utils::Pointf32) -> &mut Self {
        for e in self.enemies.iter_mut() {
            e.distance_from_target = utils::euclidean_distance(target, &e.position) - 1.;
            e.position = utils::get_rotation_angle(
                target,
                &e.position,
                utils::angle_btw_2_points(target, &e.position),
                Some(e.distance_from_target),
            );
        }
        self.update_enemies();
        self
    }
}
