use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color},
};

#[path = "../utils/mod.rs"]
mod utils;

#[derive(Debug)]
struct Projectile {
    position: utils::Pointf32,
    origin: utils::Pointf32,
    original_angle: f32,
    distance: f32,
}

#[derive(Debug)]
pub struct Projectiles {
    projectiles: Vec<Projectile>,
}
impl Projectiles {
    pub fn new() -> Self {
        Projectiles {
            projectiles: vec![],
        }
    }
    pub fn fire(&mut self, position: &utils::Pointf32, original_angle: &f32) {
        self.projectiles.push(Projectile {
            origin: *position,
            position: (0., 0.),
            original_angle: *original_angle,
            distance: -1., //utils::euclidean_distance(&origin, position),
        });
    }

    pub fn update(&mut self) {
        for p in self.projectiles.iter_mut() {
            p.distance = p.distance + 8.;
            p.position = utils::get_rotation_angle(
                &p.origin,
                &p.position,
                p.original_angle,
                Some(p.distance),
            );
        }
    }

    pub fn draw(&mut self, ctx: &ggez::Context, canvas: &mut Canvas) {
        for p in self.projectiles.iter_mut() {
            let mut fixtues = [(Color::YELLOW, p.position)];
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
