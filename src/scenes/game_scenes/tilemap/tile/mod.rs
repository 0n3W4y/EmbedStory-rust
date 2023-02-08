use serde::{ Serialize, Deserialize };

use crate::resources::scene_data::objects::{ thing::ThingType, scene_effect::SceneEffectType, character::CharacterType, stuff::StuffType };

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug )]
pub enum GroundType{
    Earth,
    DryEarth,
    Dirt,
    Rock,
    RockEnvironment,
    Clay
}

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug )]
pub enum CoverType{
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

#[derive( Serialize, Deserialize, Debug, Clone )]
pub struct Tile{
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub thing_type: Option<ThingType>,
    pub thing_id: usize,
    pub stuff_type: Option<StuffType>,
    pub stuff_id: usize,
    pub character_type: Option<CharacterType>,
    pub character_id: usize,
    pub effect_type: Option<SceneEffectType>,
    pub effect_id: usize,
    pub x: u16,
    pub y: u16,
    pub graphic_x: u32,
    pub graphic_y: u32,
    pub index: usize, // in vec;
    pub cover_graphic_index: u8,
    pub can_remove_floor: bool,
    pub can_place_floor: bool,
    pub can_place_thing: bool,
    pub can_place_stuff: bool,
    pub can_walk: bool,
    pub movement_ratio: u16,
    pub have_fog: bool,
    pub have_roof: bool,
}

impl Tile{
    pub fn new() -> Self{
        //Default Earth;
        return Tile { 
            ground_type: GroundType::Earth, 
            cover_type: CoverType::None,
            thing_type: None,
            thing_id: 0,
            stuff_type: None,
            stuff_id: 0,
            character_type: None,
            character_id: 0,
            effect_type: None,
            effect_id: 0,
            x: 0, 
            y: 0, 
            graphic_x: 0, 
            graphic_y: 0, 
            index: 0,
            cover_graphic_index: 0,
            can_remove_floor: false, 
            can_place_floor: true, 
            can_place_thing: true, 
            can_place_stuff: true, 
            can_walk: true, 
            movement_ratio: 900, 
            have_fog: false,
            have_roof: false,
        }
    }
}

#[derive( Deserialize, Clone, Debug )]
pub struct TileDeploy{
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub can_remove_floor: bool,
    pub can_place_floor: bool,
    pub can_place_thing: bool,
    pub can_place_stuff: bool,
    pub can_walk: bool,
    pub movement_ratio: u16,
}