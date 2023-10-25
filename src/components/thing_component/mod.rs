use bevy::prelude::*;
use std::collections::HashMap;

pub mod thing_animation_component;

use crate::resources::scene_data::charactor::stats::Stat;
use crate::resources::scene_data::stuff::resists_types::ResistType;
use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::resources::scene_data::thing::ThingType;
use crate::resources::scene_data::thing::ThingPermissions;

#[derive(Component, Default)]
pub struct ThingComponent{
    pub id: usize,
    pub thing_type: ThingType,
    pub graphic_index: u8,
    pub tile_index: usize,
}

#[derive(Component, Default)]
pub struct ThingPositionComponent {
    pub position: Position<i32>,
    pub permissions: Vec<ThingPermissions>,
}

#[derive(Component, Default)]
pub struct ThingStatsComponent {
    pub stats: HashMap<Stat, i16>,
    pub resists: HashMap<ResistType, i16>,
}