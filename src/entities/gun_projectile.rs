use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color},
};

#[path = "../utils/mod.rs"]
mod utils;

#[derive(Debug)]
pub struct Projectile {
    x: f32,
    y: f32,
    origin: (f32, f32),
    original_angle: f32,
    distance: f32,
}
impl Projectile {
    pub fn new(position: &utils::Pointf32, origin: &(f32, f32), original_angle: &f32) -> Self {
        let p = Projectile {
            origin: origin.clone(),
            x: position.0.clone(),
            y: position.1.clone(),
            original_angle: original_angle.clone(),
            distance: utils::euclidean_distance(&origin, position),
        };
        p
    }

    pub fn update(&mut self) -> &mut Self {
        self.distance = self.distance + 8.;
        (self.x, self.y) = utils::get_rotation_angle(
            &self.origin,
            &(self.x, self.y),
            self.original_angle,
            Some(self.distance),
        );
        return self;
    }

    pub fn draw(&self, ctx: &ggez::Context, canvas: &mut Canvas) {
        let mut fixtues = [(Color::YELLOW, (self.x, self.y))];
        for i in fixtues.iter_mut() {
            canvas.draw(
                &graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Vec2::new(0.0, 0.0),
                    10.0,
                    2.0,
                    i.0,
                )
                .unwrap(),
                Vec2::new(i.1 .0 as f32, i.1 .1 as f32),
            );
        }
    }
}
