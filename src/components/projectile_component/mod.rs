use bevy::prelude::*;
use std::collections::HashMap;

use crate::{resources::scene_data::{stuff::damage_type::DamageType, charactor::{effects::EffectType, skills::Skill}, projectiles::ProjectileType}, scenes::game_scenes::tilemap::tile::Position};

#[derive( Default, Debug, Component, Clone)]
pub struct Projectile {
    pub projectile_type: ProjectileType,
    pub damage: HashMap<DamageType, i16>,
    pub effects: Vec<EffectType>,
    pub passive_skills: Vec<Skill>,

    pub starting_position: Position<i32>,
    pub destination_point: Position<i32>,
    pub is_missed: bool,
    pub can_pierce: bool,
    pub pierce_chance: u8,
    pub velocity: u16,
}