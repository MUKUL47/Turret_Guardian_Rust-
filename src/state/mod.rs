use std::ops::Deref;

use enemy::utils;

mod enemy;
mod gun_projectile;
mod player;

pub struct State_Manager {
    pub enemies: enemy::Enemies,
    pub gun_projectile: gun_projectile::Projectiles,
    pub player: player::Player,
    pub enemies_killed: i64,
}
impl State_Manager {
    pub fn new() -> Self {
        State_Manager {
            enemies: enemy::Enemies::new(),
            gun_projectile: gun_projectile::Projectiles::new(),
            player: player::Player::new(),
            enemies_killed: 0,
        }
    }

    pub fn shoot_projectile(&mut self) -> &mut Self {
        for gun in self.player.turrets.iter() {
            self.gun_projectile.fire(&gun.gun_position, &gun.angle);
        }
        self
    }

    pub fn update_enemy(&mut self) {
        self.enemies.update(&self.player.position);
        self.enemies
            .check_projectile_collision(&mut self.gun_projectile);
    }

    pub fn update_win_size(&mut self, p: utils::Pointf32) {
        self.enemies.win_size = p;
        self.gun_projectile.win_size = p;
        self.enemies.win_size = p;
    }

    pub fn check_game_over(&mut self) {
        for e in self.enemies.enemies.iter() {
            if f32::abs(e.position.0 - self.player.position.0) < 4.
                && f32::abs(e.position.1 - self.player.position.1) < 4.
            {
                panic!("Final Score : {:?}", self.enemies.enemies_killed);
            }
        }
    }
}
