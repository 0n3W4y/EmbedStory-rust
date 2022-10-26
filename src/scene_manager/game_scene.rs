mod tilemap;
use tilemap::*;
use tile::*;
use bevy::{
    prelude::*,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive( Copy, Clone, PartialEq, Eq, Deserialize )]
pub enum RiverType{
    Horizontal,
    Vertical,
    Generate,
}

pub enum SceneType{
    Loader,
    GlobalMap,
    BattleMap,
}

pub enum SceneSubType{
    ForegroundMap,
    UndergroundMap,
}

#[derive( Clone, Deserialize )]
pub struct SceneDeployConfig{
    pub width:u16,
    pub height:u16,
    pub tile_size:u16,
    pub biome:Biome,
    pub deploy:&deploy::Deploy,
}

#[derive( Clone, Deserialize )]
pub enum Biome{
    Plain,
    Desert,
    Forest,
    Swamp,
    Winter,
    Rocks,
    Tropics,
    Nothing,
}

#[derive( Deserialize )]
pub struct AdditionalGround( tile::GroundType, u8 );
#[derive( Deserialize )]
pub struct AdditionalCover( tile::CoverType, u8 );
#[derive( Deserialize )]
pub struct River( u8, RiverConfig );
#[derive( Deserialize )]
pub struct Lake( u8, LakeConfig );
#[derive( Deserialize )]
pub struct SolidRock( u8, SolidConfig );

#[derive( Deserialize )]
pub struct RiverConfig{
    pub emerging: u8,
    pub widthMax: u16,
    pub widthMin: u16,
    pub offset: u16,
    pub widthOffset: u16,
    pub river_type: RiverType,
    pub cover: tile::CoverType,
}

#[derive( Deserialize )]
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

#[derive( Deserialize )]
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

#[derive( Deserialize )]
pub struct BiomeConfigLiquids {
    pub river: Vec<River>,
    pub lake: Vec<Lake>,
}

#[derive( Deserialize )]
pub struct BiomeConfigSolids {
    pub rock: Vec<SolidRock>,
}

#[derive( Deserialize )]
pub struct BiomeDeployConfig{
    pub groud_type:tile::GroundType,
    pub ground_type_additional:Vec<AdditionalGround>,
    pub cover_type:tile::CoverType,
    pub cover_type_additional:Vec<AdditionalCover>,
    pub liquids: BiomeConfigLiquids,
    pub solids: BiomeConfigSolids,
}



pub struct Scene{
    pub scene_type: SceneType,
    pub scene_sub_type: SceneSubType,
    pub scene_id: u32,
    pub tilemap: Tilemap,
    pub biome: Biome,
    pub objects: Vec<Entity>,
    pub stuff: Vec<Entity>,
    pub characters: Vec<Entity>,
    pub effects: Vec<Entity>,

    deploy: &deploy::Deploy,
}

impl Scene{
    pub fn generate( &self ){
        let biome_config = self.deploy.get_biome_config( &self.biome );
        self.tilemap.generate_tilemap( biome_config );
        self.generate_objects();
        //TODO: generate objects, generate characters, generate staff etc.
    }

    fn generate_objects( &self ){
        for mut tile in &self.tilemap.tiles {
            let tile_ground = tile.ground_type;
        }
    }
}

pub fn new( config: &SceneDeployConfig, id: u32  ) ->Scene {
    let tilemap_config = TilemapConfig {
        width: config.width,
        height: config.height,
        tile_size: config.tile_size,
    };
    return Scene {
        scene_type: config.scene_type,
        scene_sub_type: config.scene_sub_type,
        scene_id: id, 
        tilemap: create_tile_map( tilemap_config ),
        biome: config.biome.clone(),
        deploy: &config.deploy,
        objects: vec![],
        stuff: vec![],
        characters: vec![],
        effetcs: vec![],
    }
}

fn create_tile_map( config: TilemapConfig ) -> Tilemap {
    return tilemap::new( config );
}