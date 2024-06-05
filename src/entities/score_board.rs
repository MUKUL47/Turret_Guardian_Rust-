use std::{cell::RefCell, rc::Rc};

use enemies::utils::Pointf32;
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color, DrawParam, Drawable, PxScale, Text},
};

use crate::core::game::state::State_Manager;

#[path = "./enemies.rs"]
pub(crate) mod enemies;
#[path = "./gun_projectile.rs"]
mod gun_projectile;
#[path = "../utils/mod.rs"]
pub(crate) mod utils;

pub struct ScoreBoard {
    state_manager: Rc<RefCell<State_Manager>>,
}
impl ScoreBoard {
    pub fn new(state_manager: Rc<RefCell<State_Manager>>) -> Self {
        ScoreBoard { state_manager }
    }

    pub fn draw(&mut self, ctx: &ggez::Context, canvas: &mut Canvas) {
        let c: String = String::from("Score: ")
            + &self
                .state_manager
                .borrow()
                .enemies
                .enemies_killed
                .to_string();
        let mut text = Text::new(c);
        text.set_scale(PxScale { x: 30., y: 40. });
        canvas.draw(&text, Vec2::new(10., 10.))
    }
}
