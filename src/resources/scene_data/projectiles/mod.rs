use std::collections::HashMap;

use crate::scenes::game_scenes::tilemap::tile::Position;

use super::{stuff::damage_type::DamageType, charactor::effects::EffectType};

pub enum ProjectileType {
    Arrow,
    Bullet,
    
}

pub struct Projectile {
    projectile_type: ProjectileType,
    damage: HashMap<DamageType, u16>,
    active_effect: Vec<EffectType>,
    passive_effect: Vec<EffectType>,

    starting_position: Position<i32>,
    destination_point: Position<i32>,
}