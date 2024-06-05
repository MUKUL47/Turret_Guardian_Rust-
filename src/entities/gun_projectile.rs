use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color},
};
use utils::get_rand_color;

use crate::core::game::state::State_Manager;

#[path = "../utils/mod.rs"]
mod utils;

pub struct Projectiles {
    state_manager: Rc<RefCell<State_Manager>>,
}
impl Projectiles {
    pub fn new(state_manager: Rc<RefCell<State_Manager>>) -> Self {
        Projectiles { state_manager }
    }
    pub fn fire(&mut self) {
        self.state_manager.borrow_mut().shoot_projectile();
    }

    pub fn update(&mut self) {
        self.state_manager.borrow_mut().gun_projectile.update();
    }

    pub fn draw(&mut self, ctx: &ggez::Context, canvas: &mut Canvas) {
        for p in &self.state_manager.borrow().gun_projectile.projectiles {
            let  fixtues = [(p.color, p.position)];
            for i in fixtues.iter() {
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
