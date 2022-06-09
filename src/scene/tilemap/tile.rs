use bevy::{
    prelude::*,
};

#[derive( Clone, Reflect, Debug, PartialEq )]
pub enum GroundType {
    None,
    Earth,
    Rock,
    Sandrock,
    DryEarth,
    Dirt,
    RockEnvirounment,
    SandrockEnvirounment, 
}

#[derive( Clone, Reflect, Debug, PartialEq )]
pub enum CoverType {
    Nothing,
    Grass,
    Snow,
    Water,
    Sand,
    WoodenFloor,
    Ice,
    Shallow,
}

#[derive( Clone )]
pub struct TileDeployConfig{
    pub walkable:bool,
    pub movement_ratio:u16,
    pub place_cover:bool,
    pub place_object:bool,
    pub remove_cover:bool,
    pub remove_object:bool,
}


#[derive( Component, Clone, Copy, PartialEq, Eq )]
pub struct TileData{
    pub index: u32,
}


pub struct TileConfig{
    pub ground: GroundType,
    pub cover: CoverType,
    pub pos_x: u16,
    pub pos_y: u16,
    pub graph_x: u32,
    pub graph_y: u32,
    pub index: u32,
    pub tile_size: u16,    
    pub walkable: bool,
    pub movement_ratio: u16,
    pub place_cover: bool,
    pub place_object: bool,
    pub remove_cover: bool,
    pub remove_object: bool,    
    pub cover_graphics_index: u8,
}

#[derive( Clone, Reflect )]
pub struct Tile{
    pub ground_type:GroundType,
    pub cover_type:CoverType,
    pub x:u16,
    pub y:u16,
    pub g_x:u32, // graphics position
    pub g_y:u32, // graphics position
    pub is_walkable:bool,    
    pub movement_speed_ratio:u16,
    pub can_place_cover:bool,
    pub can_remove_cover:bool,
    pub can_place_object:bool,
    pub can_remove_object:bool,
    pub tile_size:u16,
    pub index:u32,
    pub cover_graphics_index: u8,
}


pub fn new( 
    config: TileConfig
) -> Tile{
    return Tile{
        x: config.pos_x,
        y: config.pos_y,
        g_x: config.graph_x,
        g_y: config.graph_y,
        tile_size: config.tile_size,
        is_walkable: config.walkable,
        ground_type: config.ground,
        cover_type: config.cover,
        movement_speed_ratio: config.movement_ratio,
        can_place_cover: config.place_cover,
        can_remove_cover: config.remove_cover,
        can_place_object: config.place_object,
        can_remove_object: config.remove_object,
        index: config.index,
        cover_graphics_index: config.cover_graphics_index,
    }            
}