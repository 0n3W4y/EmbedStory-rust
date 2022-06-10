pub mod tilemap;
use tilemap::*;
use bevy::{
    prelude::*,
};

#[derive( Copy, Clone, PartialEq, Eq )]
pub enum RiverType{
    Horizontal,
    Vertical,
    Generate,
}

#[derive( Clone )]
pub enum Biome{
    Plain,
    Desert,
    Forest,
    Swamp,
    Winter,
    Rocks,
    Tropics,
}

pub struct AdditionalGround( tile::GroundType, u8 );
pub struct AdditionalCover( tile::CoverType, u8 );
pub struct River( u8, RiverConfig );
pub struct Lake( u8, LakeConfig );
pub struct SolidRock( u8, SolidConfig );

pub struct RiverConfig{
    pub emerging: u8,
    pub widthMax: u16,
    pub widthMin: u16,
    pub offset: u16,
    pub widthOffset: u16,
    pub river_type: RiverType,
    pub cover: tile::CoverType,
}

pub struct LakeConfig{
    pub emerging: u8,
    pub amount: u16,
    pub widthMax: u16,
    pub widthMin: u16,
    pub heightMax: u16,
    pub heightMin: u16,
    pub offsetX: u16,
    pub offsetY: u16,
    pub widthOffset: u16,
    pub heightOffset: u16,
    pub cover: tile::CoverType,
}

pub struct SolidConfig{
    pub emerging: u8,
    pub amount: u16,
    pub widthMax: u16,
    pub widthMin: u16,
    pub heightMax: u16,
    pub heightMin: u16,
    pub offsetX: u16,
    pub offsetY: u16,
    pub widthOffset: u16,
    pub heightOffset: u16,
    pub ground: tile::GroundType,
}

pub struct BiomeConfigLiquids {
    pub river: Vec<River>,
    pub lake: Vec<Lake>,
}

pub struct BiomeConfigSolids {
    pub rock: Vec<SolidRock>,
}
pub struct BiomeConfig{
    pub groud_type:tile::GroundType,
    pub ground_type_additional:Vec<AdditionalGround>,
    pub cover_type:tile::CoverType,
    pub cover_type_additional:Vec<AdditionalCover>,
    pub liquids: BiomeConfigLiquids,
    pub solids: BiomeConfigSolids,
}



pub struct Scene{
    pub tilemap: Option,
    pub biome: Option,
}

impl Scene{
    
}

pub fn new() ->Scene{
    return Scene {
        tilemap: tilemap,
    }
}