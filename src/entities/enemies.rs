use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color},
};
use rand::*;
use utils::shared_lazy::{WINDOW_HEIGHT, WINDOW_WIDTH};
#[path = "../utils/mod.rs"]
mod utils;
enum EnemyVariant {
    EASY,
    MEDIUM,
}
pub struct Enemy {
    position: utils::Pointf32,
    distance_from_target: f32,
}

pub struct Enemies {
    enemies: Vec<Enemy>,
    c: i32,
}

impl Enemies {
    pub fn new() -> Self {
        Enemies {
            enemies: vec![],
            c: 0,
        }
    }

    fn update_enemies(&mut self) {
        self.c = i32::wrapping_add(self.c, 1);
        if self.c % 2 == 0 {
            let h = *WINDOW_HEIGHT.lock().unwrap();
            let w = *WINDOW_WIDTH.lock().unwrap();
            let mut rng = rand::thread_rng();
            let direction_rand: i32 = rng.gen_range(1..=4);
            let origin: (f32, f32) = if direction_rand == 1 {
                (0., rng.gen_range(0..=h as i32) as f32)
            } else if direction_rand == 2{
                (rng.gen_range(0..=w as i32) as f32, 0.)
            }else if direction_rand == 3{
                (rng.gen_range(0..=w as i32) as f32, h)
            }else {
                (w, rng.gen_range(0..=h as i32) as f32)
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

    pub fn window_resize(&mut self, width: &f32, height: &f32) {}

    pub fn draw(&mut self, ctx: &ggez::Context, canvas: &mut Canvas) {
        for e in self.enemies.iter_mut() {
            let mut fixtues = [(Color::GREEN, e.position)];
            for i in fixtues.iter_mut() {
                canvas.draw(
                    &graphics::Mesh::new_circle(
                        ctx,
                        graphics::DrawMode::fill(),
                        Vec2::new(0.0, 0.0),
                        5.0,
                        2.0,
                        i.0,
                    )
                    .unwrap(),
                    Vec2::new(i.1 .0 as f32, i.1 .1 as f32),
                );
            }
        }
    }
}
