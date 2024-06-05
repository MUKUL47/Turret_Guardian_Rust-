#![recursion_limit = "256"]
use std::cell::Cell;
use std::cell::RefCell;
use std::ops::Deref;
use std::path;
use std::rc::Rc;
use ggez::conf;
use ggez::conf::FullscreenType;
use ggez::event;
use ggez::glam::*;
use ggez::graphics::DrawParam;
use ggez::graphics::Drawable;
use ggez::graphics::Text;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use state::State_Manager;
#[path = "../entities/mod.rs"]
mod entities;
#[path = "../utils/mod.rs"]
mod utils;
#[path = "../state/mod.rs"]
mod state;

pub struct MainState {
    player_entity: entities::player::Player,
    enemy_entity: entities::enemies::Enemies,
    gun_entity: entities::gun_projectile::Projectiles,
    w: conf::WindowMode,
    state_manager: Rc<RefCell<State_Manager>>
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let  state_manager: Rc<RefCell<State_Manager>> = Rc::new(RefCell::new(State_Manager::new()));
        let s = MainState {
            player_entity: entities::player::Player::new(Rc::clone(&state_manager)),
            enemy_entity: entities::enemies::Enemies::new(Rc::clone(&state_manager)),
            gun_entity: entities::gun_projectile::Projectiles::new(Rc::clone(&state_manager)),
            state_manager: Rc::clone(&state_manager),
            w: conf::WindowMode {
                width: 800.,
                height: 800.,
                maximized: false,
                fullscreen_type: FullscreenType::True,
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
        self.player_entity.mouse_motion_event(_ctx, _x, _y, _dx, _dy)?;
        Ok(())
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.player_entity.update();
        self.enemy_entity.update();
        self.gun_entity.update();
        Ok(())
    }

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) -> Result<(), ggez::GameError> {
        self.player_entity.mouse_click();
        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) -> Result<(), ggez::GameError> {
        self.state_manager.borrow_mut().update_win_size((_width, _height));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        self.player_entity.draw(&ctx, &mut canvas);
        self.enemy_entity.draw(&ctx, &mut canvas);
        self.gun_entity.draw(&ctx, &mut canvas);

        canvas.finish(ctx)?;
        Ok(())
    }
}
