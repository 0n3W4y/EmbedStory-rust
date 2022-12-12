use serde::{ Serialize, Deserialize };

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize )]
pub enum GroundType{
    Earth,
    DryEarth,
    Dirt,
    Rock,
    RockEnvirounment,
}

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize )]
pub enum CoverType{
    None,
    Grass,
    Water,
    Sand,
    Snow,
    Ice,
    Shallow,
    WoodenFloor,
    RockyRoad,
}

#[derive( Serialize, Deserialize )]
pub struct GroundTilemapTile{
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub x: u16,
    pub y: u16,
    pub graphic_x: u32,
    pub graphic_y: u32,
    pub index: u32,
    pub can_remove_floor: bool,
    pub can_place_floor: bool,
    pub can_place_object: bool,
    pub can_remove_object: bool,
    pub can_place_stuff: bool,
    pub can_player_walk: bool,
    pub movement_ratio: f32,
    pub has_fog: bool,
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
            can_remove_floor: false, 
            can_place_floor: true, 
            can_place_object: true, 
            can_remove_object: false, 
            can_place_stuff: true, 
            can_player_walk: true, 
            movement_ratio: 900.0, 
            has_fog: false, 
        }
    }
}