use serde::{ Deserialize };
use std::fs::File;
use std::io::prelude::*;

use crate::resources::tilemap::tile::ground_tilemap_tile::{ GroundType, CoverType, GroundTilemapTileDeploy};

#[derive( Deserialize, Debug )]
pub struct GroundSceneTileDeploy{
    ground_tile: GroundTileDeploy,
    cover_tile: CoverTileDeploy,
}
impl GroundSceneTileDeploy{
    pub fn new( ground_path: &str, cover_path: &str ) -> Self{
        let cover_data:CoverTileDeploy  = match File::open( cover_path ){
            Ok( mut file ) => {
                let mut contents = String::new();
                file.read_to_string( &mut contents ).unwrap();
                serde_json::from_str( &contents ).expect( "JSON was not well-formatted" )
            }
            Err( err ) => panic!( "Can not open cover data file: {}, {}", err, cover_path ),
        };

        let ground_data:GroundTileDeploy  = match File::open( ground_path ){
            Ok( mut file ) => {
                let mut contents = String::new();
                file.read_to_string( &mut contents ).unwrap();
                serde_json::from_str( &contents ).expect( "JSON was not well-formatted" )
            }
            Err( err ) => panic!( "Can not open cover data file: {}, {}", err, ground_path ),
        };

        return GroundSceneTileDeploy { 
            ground_tile: ground_data, 
            cover_tile: cover_data,
        };

    }
    
    pub fn get_ground_tile_deploy( &self, value: &GroundType ) -> &GroundTilemapTileDeploy{
        return match *value{
            GroundType::Earth => &self.ground_tile.earth,
            GroundType::Dirt => &self.ground_tile.dirt,
            GroundType::DryEarth => &self.ground_tile.dry_earth,
            GroundType::Rock => &self.ground_tile.rock,
            GroundType::RockEnvironment => &self.ground_tile.rock_environment,
        };
    }

    pub fn get_cover_tile_deploy( &self, value: &CoverType ) -> &GroundTilemapTileDeploy{
        return match *value{
            CoverType::Grass => &self.cover_tile.grass,
            CoverType::None => &self.cover_tile.none,
            CoverType::Flowers => &self.cover_tile.flowers,
            CoverType::Ice => &self.cover_tile.ice,
            CoverType::Sand => &self.cover_tile.sand,
            CoverType::Shallow => &self.cover_tile.shallow,
            CoverType::Snow => &self.cover_tile.snow,
            CoverType::Water => &self.cover_tile.water,
            CoverType::RockyRoad => &self.cover_tile.rocky_road,
            CoverType::WoodenFloor => &self.cover_tile.wooden_floor,
        }
    }
}

#[derive( Deserialize, Debug )]
pub struct GroundTileDeploy{
    earth: GroundTilemapTileDeploy,
    dirt: GroundTilemapTileDeploy,
    dry_earth: GroundTilemapTileDeploy,
    rock: GroundTilemapTileDeploy,
    rock_environment: GroundTilemapTileDeploy,
}

#[derive( Deserialize, Debug )]
pub struct CoverTileDeploy{
    none: GroundTilemapTileDeploy,
    grass: GroundTilemapTileDeploy,
    flowers: GroundTilemapTileDeploy,
    water: GroundTilemapTileDeploy,
    sand: GroundTilemapTileDeploy,
    snow: GroundTilemapTileDeploy,
    ice: GroundTilemapTileDeploy,
    shallow: GroundTilemapTileDeploy,
    wooden_floor: GroundTilemapTileDeploy,
    rocky_road: GroundTilemapTileDeploy,
}