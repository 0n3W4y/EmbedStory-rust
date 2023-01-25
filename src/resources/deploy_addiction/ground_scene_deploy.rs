use serde::{ Deserialize };
use std::fs::File;
use std::io::prelude::*;

use crate::resources::deploy_addiction::ground_scene_biome_deploy::BiomeType;

#[derive( Deserialize, Debug )]
pub struct GroundSceneDeployConfig{
    biome_type: BiomeType,
    width: u16,
    height: u16,
    underground: u8, // percent;
    underground_floor: u8, // min 0 - max - value;
}

#[derive( Deserialize, Debug )]
pub struct GroundSceneDeploy{
    plain_event: GroundSceneDeployConfig,
}

impl GroundSceneDeploy{
    pub fn new( path: &str ) -> Self{
        let result: GroundSceneDeploy  = match File::open( path ){
            Ok( mut file ) => {
                let mut contents = String::new();
                file.read_to_string( &mut contents ).unwrap();
                serde_json::from_str( &contents ).expect( "JSON was not well-formatted" )
            }
            Err( err ) => panic!( "Can not open ground scene data file: {}, {}", err, path ),
        };

        return result;
    }
}