use ggez::{
    event,
    glam::Vec2,
    graphics::{self, Canvas, Color, Mesh},
    GameResult,
};

#[path = "./gun_projectile.rs"]
mod gun_projectile;
#[path = "../utils/mod.rs"]
mod utils;
//
static TURRET_DISTANCE: f32 = 50.;
pub struct Player {
    position: utils::Pointf32,
    angle: f32,
    gun_position: utils::Pointf32,
    gun_projectiles: Vec<gun_projectile::Projectile>,
}
impl Player {
    pub fn new() -> Self {
        Player {
            position: (1., 1.),
            gun_position: (1., 1.),
            angle: (0.),
            gun_projectiles: vec![],
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
        for i in self.gun_projectiles.iter_mut() {
            i.update();
        }
        return self;
    }

    pub fn mouse_motion_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), ggez::GameError> {
        self.position = (_x, _y);
        Ok(())
    }

    pub fn mouse_click(&mut self) -> &mut Self {
        self.gun_projectiles.push(gun_projectile::Projectile::new(
            &self.gun_position,
            &self.position,
            &self.angle
        ));
        self
    }

    pub fn draw(&self, ctx: &ggez::Context, canvas: &mut Canvas) {
        let mut fixtues = [
            (Color::RED, self.position),
            (Color::WHITE, self.gun_position),
        ];
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
        self.draw_projectiles(ctx, canvas);
    }

    fn draw_projectiles(&self, ctx: &ggez::Context, canvas: &mut Canvas) {
        for i in self.gun_projectiles.iter() {
            i.draw(ctx, canvas);
        }
    }
}
