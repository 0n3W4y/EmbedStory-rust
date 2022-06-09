pub mod tilemap;
use tilemap::*;
use bevy::{
    prelude::*,
};

pub enum RiverType{
    Horizontal,
    Vertical,
    Generate,
}

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
pub struct Solid( u8, SolidConfig );

pub struct RiverConfig{
    pub emerging: u8,
    pub widthMax: u8,
    pub widthMin: u8,
    pub offset: u8,
    pub widthOffset: u8,
    pub river_type: RiverType,
    pub cover: tile::CoverType,
}

pub struct LakeConfig{
    pub emerging: u8,
    pub amount: u8,
    pub widthMax: u8,
    pub widthMin: u8,
    pub heightMax: u8,
    pub heightMin: u8,
    pub offsetX: u8,
    pub offsetY: u8,
    pub widthOffset: u8,
    pub heightOffset: u8,
    pub floorType: tile::CoverType,
}

pub struct SolidConfig{
    emerging: u8,
    amount: u8,
    widthMax: u8,
    widthMin: u8,
    heightMax: u8,
    heightMin: u8,
    offsetX: u8,
    offsetY: u8,
    widthOffset: u8,
    heightOffset: u8,
    groundType: tile::GroundType,
}

pub struct BiomeConfigLiquids{
    pub river: Vec<River>,
    pub lake: Vec<Lake>,
}

pub struct BiomeConfigSolids{
    pub solids: Vec<Solid>,
}
pub struct BiomeConfig{
    groud_type:tile::GroundType,
    ground_type_additional:Vec<AdditionalGround>,
    cover_type:tile::CoverType,
    cover_type_additional:Vec<AdditionalCover>,
    liquids: BiomeConfigLiquids,
    solids: BiomeConfigSolids,
}



#[derive( Component, Reflect)]
#[reflect(Component)]
pub struct Scene{
    pub tilemap:Tilemap,
    pub biome: Biome,
}