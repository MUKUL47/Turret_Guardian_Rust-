use std::collections::HashMap;

use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color},
};
use utils::Pointf32;

#[path = "../utils/mod.rs"]
mod utils;

#[derive(Clone, Copy)]
pub struct Projectile {
    pub position: utils::Pointf32,
    pub origin: utils::Pointf32,
    pub original_angle: f32,
    pub distance: f32,
    pub is_deleted: bool
}

impl Projectile{
    pub fn mark_deleted(&mut self){
        self.is_deleted = true
    }
}

pub struct Projectiles {
    pub projectiles: Vec<Projectile>,
    pub win_size: Pointf32,
    pub next_projectile_c: i32
}

impl Projectiles {
    pub fn new() -> Self {
        Projectiles {
            projectiles: Vec::new(),
            win_size: (0., 0.),
            next_projectile_c: 0
        }
    }
    pub fn fire(&mut self, position: &utils::Pointf32, original_angle: &f32) {
        self.next_projectile_c = self.next_projectile_c.wrapping_add(1);
        self.projectiles.push(Projectile {
            origin: *position,
            position: (0., 0.),
            original_angle: *original_angle,
            distance: -1., //utils::euclidean_distance(&origin, position),
            is_deleted: false
        });
    }

    pub fn update(&mut self) {
        self.projectiles.retain_mut(|p| {
            let x = p.position.0;
            let y = p.position.1;
            return !(x < -1. || x > self.win_size.0 || y < -1. || y > self.win_size.1 || p.is_deleted);
        });
        for p in &mut self.projectiles.iter_mut() {
            p.distance = p.distance + 8.;
            p.position = utils::get_rotation_angle(
                &p.origin,
                &p.position,
                p.original_angle,
                Some(p.distance),
            );
        }
    }
}
