use bevy::prelude::*;
//use bevy_inspector_egui::Inspectable;

use crate::scenes::game_scenes::tilemap::tile::{GroundType, CoverType, Position, TilePermissions};
use crate::resources::scene_data::objects::{thing::ThingType, character::CharacterType, stuff::StuffType, scene_effect::SceneEffectType};

#[derive(Component)]
pub struct TileComponent{
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub index: usize, // vec index in tilemapstorage;
    pub cover_graphic_index: u8,
    pub movement_ratio: u16,

    pub position: Position<i32>,
    pub graphic_position: Position<f32>,

    pub permissions: Vec<TilePermissions>,

    pub thing_type: Option<(ThingType, usize)>, // ( thing type, id of thing);
    pub stuff_type:  Option<(StuffType, usize)>,
    pub character_type:  Option<(CharacterType, usize)>,
    pub effect_type: Option<(SceneEffectType, usize)>
}