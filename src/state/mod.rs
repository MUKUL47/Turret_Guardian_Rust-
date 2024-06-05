use std::ops::Deref;

use enemy::utils;

mod enemy;
mod gun_projectile;
mod player;

pub struct State_Manager {
    pub enemies: enemy::Enemies,
    pub gun_projectile: gun_projectile::Projectiles,
    pub player: player::Player
}
impl State_Manager {
    pub fn new() -> Self {
        State_Manager {
            enemies: enemy::Enemies::new(),
            gun_projectile: gun_projectile::Projectiles::new(),
            player: player::Player::new()
        }
    }

    pub fn shoot_projectile(&mut self) -> &mut Self {
        self.gun_projectile
            .fire(&self.player.position, &self.player.angle);
        self
    }

    pub fn update_enemy(&mut self) {
        self.enemies.update(&self.player.position);
        self.enemies.check_projectile_collision(&mut self.gun_projectile);
    }

    pub fn update_win_size(&mut self, p: utils::Pointf32){
        self.enemies.win_size = p;
        self.gun_projectile.win_size = p;
        self.enemies.win_size = p;
    }
}
