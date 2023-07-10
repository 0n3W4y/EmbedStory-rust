use bevy::prelude::*;
use std::collections::HashMap;

pub mod thing_animation_component;

use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::resources::scene_data::thing::ThingType;
use crate::resources::scene_data::thing::ThingPermissions;

#[derive(Component, Default)]
pub struct ThingComponent{
    pub id: usize,
    pub tile_index: usize,
    pub thing_type: ThingType,
    pub graphic_index: u8,

    
    pub current_health_points: i16,
    pub total_health_points: i16
}

#[derive(Component, Default)]
pub struct ThingPositionComponent {
    pub position: Position<i32>,
    pub permissions: Vec<ThingPermissions>,
}

#[derive(Component, Default)]
pub struct ThintStatsComponent {

}