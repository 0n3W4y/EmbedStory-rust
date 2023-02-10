use serde::{Deserialize, Serialize};

use crate::resources::scene_data::objects::{
    character::CharacterType, scene_effect::SceneEffectType, stuff::StuffType, thing::ThingType,
};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum GroundType {
    Earth,
    DryEarth,
    Dirt,
    Rock,
    RockEnvironment,
    Clay,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy)]
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
pub struct Position{
    pub x: i32, 
    pub y: i32
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Tile {
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub index: usize, // in vec;

    pub position: Position,
    pub graphic_position: Position,

    pub cover_graphic_index: u8,
    pub can_remove_floor: bool,
    pub can_place_floor: bool,
    pub can_place_thing: bool,
    pub can_place_stuff: bool,
    pub can_walk: bool,
    pub movement_ratio: u16,
    pub have_fog: bool,
    pub have_roof: bool,

    pub thing_type: Option<ThingType>,
    pub thing_id: usize,
    pub stuff_type: Option<StuffType>,
    pub stuff_id: usize,
    pub character_type: Option<CharacterType>,
    pub character_id: usize,
    pub effect_type: Option<SceneEffectType>,
    pub effect_id: usize,
}

impl Tile {
    pub fn new(
        index: usize, 
        x: i32, 
        y: i32, 
        graphic_x: i32, 
        graphic_y: i32, 
        ground_type: GroundType,
        can_remove_floor: bool,
        can_place_floor: bool,
        can_place_thing: bool,
        can_place_stuff: bool,
        can_walk: bool,
        have_fog: bool,
        have_roof: bool,
        movement_ratio: u16
    ) -> Self {
        //Default Earth;
        return Tile {
            ground_type: ground_type,
            cover_type: CoverType::None,
            position: Position{x: x, y: y},
            graphic_position: Position{x: graphic_x, y: graphic_y},
            index: index,
            thing_type: None,
            thing_id: 0,
            stuff_type: None,
            stuff_id: 0,
            character_type: None,
            character_id: 0,
            effect_type: None,
            effect_id: 0,
            cover_graphic_index: 0,
            can_remove_floor: false,
            can_place_floor: true,
            can_place_thing: true,
            can_place_stuff: true,
            can_walk: true,
            movement_ratio: 900,
            have_fog: false,
            have_roof: false,
        };
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct TileDeploy {
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub can_remove_floor: bool,
    pub can_place_floor: bool,
    pub can_place_thing: bool,
    pub can_place_stuff: bool,
    pub can_walk: bool,
    pub movement_ratio: u16,
    pub have_fog: bool,
    pub have_roof: bool,
}
