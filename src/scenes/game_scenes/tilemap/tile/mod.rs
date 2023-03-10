use serde::{Deserialize, Serialize};

use crate::resources::scene_data::objects::{
    charactor::CharactorType, scene_effect::SceneEffectType, stuff::StuffType, thing::ThingType,
};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Default)]
pub enum TilePermissions{
    RemoveFloor,
    PlaceFloor,
    PlaceThing,
    #[default]
    Walk,
    PlaceStuff,
    Roof,
    Fog,
    PlaceEffect,
    EndMovementPoint,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Default, Hash)]
pub enum GroundType {
    #[default]
    Earth,
    DryEarth,
    Dirt,
    Rock,
    RockEnvironment,
    Clay,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Default, Hash)]
pub enum CoverType {
    #[default]
    None,
    Grass,
    Flowers,
    Water,
    Sand,
    Snow,
    Ice,
    Shallow,
    WoodenFloor,
    RockyRoad,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub struct Position<T>{
    pub x: T, 
    pub y: T
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tile{
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub index: usize, // in vec;
    pub cover_graphic_index: u8,
    pub movement_ratio: u16,

    pub position: Position<i32>,
    pub graphic_position: Position<f32>,    

    pub permissions: Vec<TilePermissions>,

    pub thing_type: Option<(ThingType, usize)>, // ( thing type, id of thing);
    pub stuff_type: Vec<(StuffType, usize)>,
    pub alive_charactor_type: Option<(CharactorType, usize)>,
    pub dead_charactor_type: Vec<(CharactorType, usize)>,
    pub effect_type: Option<(SceneEffectType, usize)>
}

#[derive(Deserialize, Clone, Debug)]
pub struct TileDeploy {
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub movement_ratio: u16,
    pub permissions: Vec<TilePermissions>
}