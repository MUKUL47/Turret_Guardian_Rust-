mod utils;
mod core;
use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;
fn main() {
    
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build().unwrap();
    let state = core::game::MainState::new().unwrap();
    event::run(ctx, event_loop, state)
}
