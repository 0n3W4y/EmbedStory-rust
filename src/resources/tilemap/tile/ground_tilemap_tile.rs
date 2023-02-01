use serde::{ Serialize, Deserialize };

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
pub struct GroundTilemapTile{
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub x: u16,
    pub y: u16,
    pub graphic_x: u32,
    pub graphic_y: u32,
    pub index: u32, // in vec;
    pub cover_graphic_index: u8,
    pub can_remove_floor: bool,
    pub can_place_floor: bool,
    pub can_place_thing: bool,
    pub can_remove_thing: bool,
    pub can_place_stuff: bool,
    pub can_walk: bool,
    pub movement_ratio: u16,
    pub have_fog: bool,
    pub have_roof: bool,
}

impl GroundTilemapTile{
    pub fn new() -> GroundTilemapTile{
        //Default Earth;
        return GroundTilemapTile { 
            ground_type: GroundType::Earth, 
            cover_type: CoverType::None, 
            x: 0, 
            y: 0, 
            graphic_x: 0, 
            graphic_y: 0, 
            index: 0,
            cover_graphic_index: 0,
            can_remove_floor: false, 
            can_place_floor: true, 
            can_place_thing: true, 
            can_remove_thing: false, 
            can_place_stuff: true, 
            can_walk: true, 
            movement_ratio: 900, 
            have_fog: false,
            have_roof: false,
        }
    }
}

#[derive( Deserialize, Clone, Debug )]
pub struct GroundTilemapTileDeploy{
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub can_remove_floor: bool,
    pub can_place_floor: bool,
    pub can_place_thing: bool,
    pub can_remove_thing: bool,
    pub can_place_stuff: bool,
    pub can_walk: bool,
    pub movement_ratio: u16,
}