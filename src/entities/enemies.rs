use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color},
};

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
}

impl Enemies {
    pub fn new() -> Self {
        let position = (800., 800.);
        let e = Enemy {
            distance_from_target: utils::euclidean_distance(&(0.,0.), &position),
            position,
        };
        let mut enemies = vec![];
        enemies.push(e);
        Enemies { enemies }
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
        self
    }

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
