use bevy::prelude::*;

pub mod thing_animation_component;

use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::resources::scene_data::objects::thing::ThingType;
use crate::resources::scene_data::objects::thing::ThingPermissions;
use crate::resources::scene_data::objects::resists::Resist;
use crate::resources::scene_data::objects::body_part::BodyPart;

#[derive(Component, Default)]
pub struct ThingComponent{
    pub id: usize,
    pub tile_index: usize, // index of tile in vec on tilemap for fast get; because all tiles r static in tilemap vec;
    pub thing_type: ThingType,

    pub position: Position<i32>,
    pub graphic_position: Position<f32>,
    pub graphic_index: u8,

    pub permissions: Vec<ThingPermissions>,
    pub resists: Vec<Resist>,
    pub resists_cache: Vec<Resist>,
    pub body_structure: Vec<BodyPart>,
    pub current_health_points: i16,
    pub total_health_points: i16
}