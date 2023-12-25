use bevy::prelude::*;
use std::collections::HashMap;

use crate::{resources::scene_data::{charactor::{effects::Effect, skills::PassiveSkill}, projectiles::ProjectileType, Damage}, scenes::game_scenes::tilemap::tile::Position};

#[derive( Default, Debug, Component, Clone)]
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
    pub is_missed: bool,
    pub is_critical_hit: bool,
    pub velocity: u16,
}