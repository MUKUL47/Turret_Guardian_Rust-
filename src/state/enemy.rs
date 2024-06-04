use rand::*;
use utils::{Pointf32};
#[path = "../utils/mod.rs"]
pub(crate) mod utils;
#[derive(Clone)]
pub struct Enemy {
    pub position: utils::Pointf32,
    distance_from_target: f32,
}
#[derive(Clone)]

pub struct Enemies {
    pub enemies: Vec<Enemy>,
    c: i32,
    pub win_size: Pointf32
}

    impl Enemies {
    pub fn new() -> Self {
        Enemies {
            enemies: vec![],
            c: 0,
            win_size: (0.,0.)
        }
    }

    fn update_enemies(&mut self) {
        self.c = i32::wrapping_add(self.c, 1);
        if self.c % 10 == 0 {
            let mut rng = rand::thread_rng();
            let direction_rand: i32 = rng.gen_range(1..=4);
            let origin: (f32, f32) = if direction_rand == 1 {
                (0., rng.gen_range(0..=self.win_size.1 as i32) as f32)
            } else if direction_rand == 2{
                (rng.gen_range(0..=self.win_size.0 as i32) as f32, 0.)
            }else if direction_rand == 3{
                (rng.gen_range(0..=self.win_size.0 as i32) as f32, self.win_size.1)
            }else {
                (self.win_size.0, rng.gen_range(0..=self.win_size.1 as i32) as f32)
            };
            self.enemies.push(Enemy {
                distance_from_target: utils::euclidean_distance(&(0., 0.), &origin),
                position: origin,
            });
        }
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
