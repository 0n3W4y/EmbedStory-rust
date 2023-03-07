use bevy::prelude::*;
//use bevy_inspector_egui::Inspectable;

pub mod tile_cover_component;

use crate::resources::scene_data::objects::character::CharacterType;
use crate::resources::scene_data::objects::scene_effect::SceneEffectType;
use crate::resources::scene_data::objects::stuff::StuffType;
use crate::resources::scene_data::objects::thing::ThingType;
use crate::scenes::game_scenes::tilemap::tile::{GroundType, Position, TilePermissions};

#[derive(Component, Default)]
pub struct TileComponent {
    pub ground_type: GroundType,
    // cover_type moved to new component;
    pub index: usize, // in vec;
    pub cover_graphic_index: u8,
    pub movement_ratio: u16,

    pub position: Position<i32>,
    pub graphic_position: Position<f32>,

    pub permissions: Vec<TilePermissions>,

    pub thing_type: Option<(ThingType, usize)>,
    pub stuff_type: Vec<(StuffType, usize)>,
    pub alive_character_type: Option<(CharacterType, usize)>,
    pub dead_character_type: Vec<(CharacterType, usize)>,
    pub effect_type: Option<(SceneEffectType, usize)>,
}
