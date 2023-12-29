use bevy::prelude::*;
use std::collections::HashMap;

use crate::{resources::scene_data::{charactor::{effects::Effect, skills::PassiveSkill}, projectiles::ProjectileType, Damage}, scenes::game_scenes::tilemap::tile::Position};

#[derive(Default, Debug, Component, Clone)]
pub struct Projectile {
    pub projectile_type: ProjectileType,
    pub damage: HashMap<Damage, i16>,
    pub effects: Vec<Effect>,
    pub passive_skills: Vec<PassiveSkill>,
    pub range: u8,
    pub starting_position: Position<i32>,
    pub current_position: Position<i32>,
    pub motion_coefficient: Position<f32>,
    pub direction: Position<i8>,
    pub area_on_impact: u8,
    pub is_missed: bool,
    pub is_critical_hit: bool,
    pub velocity: u16,
}

impl Projectile {
    pub fn new(projectile_type: &ProjectileType) -> Self {
        let velocity = match *projectile_type {
            ProjectileType::Arrow => 120,
            ProjectileType::Bullet => 300,
            ProjectileType::FireSphere => 150,
            ProjectileType::None => 1000,
        };

        Projectile {
            projectile_type: projectile_type.clone(),
            damage: HashMap::new(),
            effects: vec![],
            passive_skills: vec![],
            range: 0,
            starting_position: Position {x:0, y:0},
            current_position: Position {x:0, y:0},
            motion_coefficient: Position {x:0.0, y:0.0},
            direction: Position {x:0, y:0},
            area_on_impact: 0,
            is_missed: false,
            is_critical_hit: false,
            velocity,
        }
    }
}