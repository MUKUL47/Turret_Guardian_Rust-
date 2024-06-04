use std::{cell::RefCell, rc::Rc};

use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color},
};
use rand::*;
use utils::{Pointf32};
#[path = "../utils/mod.rs"]
pub(crate) mod utils;

use crate::core::game::state::State_Manager;

pub struct Enemies {           
     state_manager: Rc<RefCell<State_Manager>>,

}

    impl Enemies {
    pub fn new(state_manager: Rc<RefCell<State_Manager>>) -> Self {
        Enemies {
            state_manager
        }
    }
    pub fn update(&mut self) {
        self.state_manager.borrow_mut().update_enemy();
    }


    pub fn draw(&mut self, ctx: &ggez::Context, canvas: &mut Canvas) {
        for e in self.state_manager.borrow_mut().enemies.enemies.iter_mut() {
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
