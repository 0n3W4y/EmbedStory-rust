use serde::{ Deserialize };
use std::fs::File;
use std::io::prelude::*;

use crate::scenes::game_scenes::tilemap::tile::{ GroundType, CoverType, TileDeploy };

#[derive( Deserialize, Debug )]
pub struct TilemapTileDeploy{
    ground_tile: GroundTileDeploy,
    cover_tile: CoverTileDeploy,
}
impl TilemapTileDeploy{
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

        return TilemapTileDeploy { 
            ground_tile: ground_data, 
            cover_tile: cover_data,
        };

    }
    
    pub fn get_ground_tile_deploy( &self, value: &GroundType ) -> &TileDeploy{
        return match *value{
            GroundType::Earth => &self.ground_tile.earth,
            GroundType::Dirt => &self.ground_tile.dirt,
            GroundType::DryEarth => &self.ground_tile.dry_earth,
            GroundType::Rock => &self.ground_tile.rock,
            GroundType::Clay => &self.ground_tile.clay,
        };
    }

    pub fn get_cover_tile_deploy( &self, value: &CoverType ) -> &TileDeploy{
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
    earth: TileDeploy,
    dirt: TileDeploy,
    dry_earth: TileDeploy,
    rock: TileDeploy,
    clay: TileDeploy,
}

#[derive( Deserialize, Debug )]
pub struct CoverTileDeploy{
    none: TileDeploy,
    grass: TileDeploy,
    flowers: TileDeploy,
    water: TileDeploy,
    sand: TileDeploy,
    snow: TileDeploy,
    ice: TileDeploy,
    shallow: TileDeploy,
    wooden_floor: TileDeploy,
    rocky_road: TileDeploy,
}