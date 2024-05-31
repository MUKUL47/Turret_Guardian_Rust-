use std::path;

use ggez::conf;
use ggez::conf::FullscreenType;
use ggez::event;
use ggez::event::EventHandler;
use ggez::glam::*;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
#[path = "../entities/mod.rs"]
mod entities;
#[path = "../utils/mod.rs"]
mod utils;
pub struct MainState {
    player: entities::player::Player,
    w: conf::WindowMode,
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let s = MainState {
            player: entities::player::Player::new(),
            w: conf::WindowMode {
                width: 800.0,
                height: 600.0,
                maximized: false,
                fullscreen_type: FullscreenType::Windowed,
                borderless: false,
                min_width: 1.0,
                max_width: 0.0,
                min_height: 1.0,
                max_height: 0.0,
                resizable: false,
                visible: true,
                transparent: false,
                resize_on_scale_factor_change: false,
                logical_size: None,
            },
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), ggez::GameError> {
        self.player.mouse_motion_event(_ctx, _x, _y, _dx, _dy)?;
        Ok(())
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.player.update();
        Ok(())
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) -> Result<(), ggez::GameError> {
        self.player.mouse_click();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        self.player.draw(&ctx, &mut canvas);

        canvas.finish(ctx)?;
        Ok(())
    }
}
