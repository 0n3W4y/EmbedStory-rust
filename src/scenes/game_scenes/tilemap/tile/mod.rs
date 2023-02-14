use serde::{Deserialize, Serialize};

use crate::resources::scene_data::objects::{
    character::CharacterType, scene_effect::SceneEffectType, stuff::StuffType, thing::ThingType,
};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub enum TilePermissions{
    RemoveFloor,
    PlaceFloor,
    PlaceThing,
    Walk,
    PlaceStuff,
    Roof,
    Fog
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub enum GroundType {
    Earth,
    DryEarth,
    Dirt,
    Rock,
    RockEnvironment,
    Clay,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub enum CoverType {
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

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct Position<T>{
    pub x: T, 
    pub y: T
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tile{
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub index: usize, // in vec;
    pub cover_graphic_index: u8,
    pub movement_ratio: u16,

    pub position: Position<i32>,
    pub graphic_position: Position<f32>,    

    pub permissions: Vec<TilePermissions>,

    pub thing_type: (Option<ThingType>, usize),
    pub stuff_type:  (Option<StuffType>, usize),
    pub character_type:  (Option<CharacterType>, usize),
    pub effect_type: (Option<SceneEffectType>, usize),
}

impl Tile {
    pub fn new(
        index: usize, 
        x: i32, 
        y: i32, 
        graphic_x: f32, 
        graphic_y: f32, 
        ground_type: GroundType,
        movement_ratio: u16,
        permissions: Vec<TilePermissions>
    ) -> Self {
        //Default Earth;
        return Tile {
            ground_type: ground_type,
            cover_type: CoverType::None,
            position: Position{x, y},
            graphic_position: Position{x: graphic_x, y: graphic_y},
            cover_graphic_index: 0,
            index,
            thing_type: (None, 0),
            stuff_type: (None, 0),
            character_type: (None, 0),
            effect_type: (None, 0),
            movement_ratio: 900,
            permissions
        };
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct TileDeploy {
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub movement_ratio: u16,
    pub permissions: Vec<TilePermissions>

}