use std::{cell::RefCell, rc::Rc};

use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color},
};

use crate::core::game::state::State_Manager;

#[path = "./enemies.rs"]
pub(crate) mod enemies;
#[path = "./gun_projectile.rs"]
mod gun_projectile;
#[path = "../utils/mod.rs"]
pub(crate) mod utils;
//
static TURRET_DISTANCE: f32 = 50.;

pub struct Player {
    state_manager: Rc<RefCell<State_Manager>>,
}
impl Player {
    pub fn new(state_manager: Rc<RefCell<State_Manager>>) -> Self {
        Player { state_manager }
    }

    pub fn mouse_motion_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), ggez::GameError> {
        self.state_manager
            .borrow_mut()
            .player
            .update_position(_x, _y);
        Ok(())
    }

    pub fn update(&mut self){
        self.state_manager.borrow_mut().player.update();
    }

    pub fn mouse_click(&mut self) -> &mut Self {
        self.state_manager.borrow_mut().shoot_projectile();
        self
    }

    pub fn draw(&mut self, ctx: &ggez::Context, canvas: &mut Canvas) {
        let state = self.state_manager.borrow_mut();
        let mut fixtues = [
            (Color::RED, state.player.position),
            (Color::WHITE, state.player.gun_position),
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
    }
}
