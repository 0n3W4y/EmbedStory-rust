use bevy::prelude::*;
use std::collections::HashMap;

pub mod thing_animation_component;

use crate::resources::scene_data::stuff::damage_type::DamageType;
use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::resources::scene_data::thing::ThingType;
use crate::resources::scene_data::thing::ThingPermissions;

#[derive(Component, Default)]
pub struct ThingComponent{
    pub id: usize,
    pub thing_type: ThingType,
    pub graphic_index: u8,
}

#[derive(Component, Default)]
pub struct ThingPositionComponent {
    pub position: Position<i32>,
    pub permissions: Vec<ThingPermissions>,
}

#[derive(Component, Default)]
pub struct ThintStatsComponent {
    pub extra_stats: HashMap<DamageType, i16>,
    pub resists: HashMap<DamageType, i16>,
}