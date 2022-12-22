use bevy::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

use crate::resources::tilemap::tile::ground_tilemap_tile::GroundTilemapTileDeploy;

use super::tilemap::tile::ground_tilemap_tile::{GroundType, CoverType};

#[derive( Deserialize, Debug )]
pub struct GroundTilemapTileDeployData{
    ground_tile_data: GroundTilemapTileGroundDeployData,
    cover_tile_data: GroundTilemapTileCoverDeployData,
}
impl GroundTilemapTileDeployData{
    pub fn new( ground_path: &str, cover_path: &str ) -> Self{
        let cover_data:GroundTilemapTileCoverDeployData  = match File::open( cover_path ){
            Ok( mut file ) => {
                let mut contents = String::new();
                file.read_to_string( &mut contents ).unwrap();
                serde_json::from_str( &contents ).expect( "JSON was not well-formatted" )
            }
            Err( err ) => panic!( "Can not open cover data file: {}, {}", err, cover_path ),
        };

        let ground_data:GroundTilemapTileGroundDeployData  = match File::open( ground_path ){
            Ok( mut file ) => {
                let mut contents = String::new();
                file.read_to_string( &mut contents ).unwrap();
                serde_json::from_str( &contents ).expect( "JSON was not well-formatted" )
            }
            Err( err ) => panic!( "Can not open cover data file: {}, {}", err, ground_path ),
        };

        return GroundTilemapTileDeployData { 
            ground_tile_data: ground_data, 
            cover_tile_data: cover_data, 
        };

    }
}

#[derive( Deserialize, Debug )]
pub struct GroundTilemapTileGroundDeployData{
    earth: GroundTilemapTileDeploy,
    dirt: GroundTilemapTileDeploy,
    dry_earth: GroundTilemapTileDeploy,
    rock: GroundTilemapTileDeploy,
    rock_envirounment: GroundTilemapTileDeploy,
}

#[derive( Deserialize, Debug )]
pub struct GroundTilemapTileCoverDeployData{
    none: GroundTilemapTileDeploy,
    grass: GroundTilemapTileDeploy,
    water: GroundTilemapTileDeploy,
    sand: GroundTilemapTileDeploy,
    snow: GroundTilemapTileDeploy,
    ice: GroundTilemapTileDeploy,
    shallow: GroundTilemapTileDeploy,
    wooden_floor: GroundTilemapTileDeploy,
    rocky_road: GroundTilemapTileDeploy,
}

#[derive( Deserialize, Debug )]
pub struct Deploy{
    pub ground_tilemap_tile_data: GroundTilemapTileDeployData,
}

impl Deploy{
    pub fn get_ground_tile_data( &self, value: &GroundType ) -> &GroundTilemapTileDeploy{
        return match *value{
            GroundType::Earth => &self.ground_tilemap_tile_data.ground_tile_data.earth,
            GroundType::Dirt => &self.ground_tilemap_tile_data.ground_tile_data.dirt,
            GroundType::DryEarth => &self.ground_tilemap_tile_data.ground_tile_data.dry_earth,
            GroundType::Rock => &self.ground_tilemap_tile_data.ground_tile_data.rock,
            GroundType::RockEnvironment => &self.ground_tilemap_tile_data.ground_tile_data.rock_envirounment,
        };
    }

    pub fn get_cover_tile_data( &self, value: &CoverType ) -> &GroundTilemapTileDeploy{
        return match *value{
            CoverType::Grass => &self.ground_tilemap_tile_data.cover_tile_data.grass,
            CoverType::None => &self.ground_tilemap_tile_data.cover_tile_data.none,
            CoverType::Ice => &self.ground_tilemap_tile_data.cover_tile_data.ice,
            CoverType::Sand => &self.ground_tilemap_tile_data.cover_tile_data.sand,
            CoverType::Shallow => &self.ground_tilemap_tile_data.cover_tile_data.shallow,
            CoverType::Snow => &self.ground_tilemap_tile_data.cover_tile_data.snow,
            CoverType::Water => &self.ground_tilemap_tile_data.cover_tile_data.water,
            CoverType::RockyRoad => &self.ground_tilemap_tile_data.cover_tile_data.rocky_road,
            CoverType::WoodenFloor => &self.ground_tilemap_tile_data.cover_tile_data.wooden_floor,
        }
    }
}

impl FromWorld for Deploy{
    fn from_world( _world: &mut World ) -> Self {
        let cover_data_path: &str = "deploy/ground_tilemap_tile_cover_config.json";
        let ground_data_path: &str = "deploy/ground_tilemap_tile_ground_config.json";
        

        let tile_data = GroundTilemapTileDeployData::new( ground_data_path, cover_data_path );
        return Deploy{
            ground_tilemap_tile_data: tile_data,
        };
    }
}