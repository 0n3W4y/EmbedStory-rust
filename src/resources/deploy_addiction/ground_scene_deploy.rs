use serde::{ Deserialize };
use std::fs::File;
use std::io::prelude::*;

use crate::resources::deploy_addiction::ground_scene_biome_deploy::BiomeType;

#[derive( Deserialize, Debug )]
pub struct GroundSceneDeployConfig{
    pub biome_type: BiomeType,
    pub width: u16,
    pub height: u16,
    pub underground: u8, // percent;
    pub underground_floor: u8, // min 0 - max - value;
    //pub house: u8,
    //pub house_count: u8,
    //pub neutral_animals: u8,
    //pub neutral_animals_count: u8,
    //pub neutral_npc: u8,
    //pub neutral_npc_count: u8,
    //pub enemy_npc: u8,
    //pub enemy_npc_count:u8,
    //pub enemy_npc_strength: Strong,
    //pub enemy_animals: u8,
    //pub enemy_animals_count: u8,
    //pub enemy_animals_strength: Weak,
}

#[derive( Deserialize, Debug )]
pub struct GroundSceneDeploy{
    pub plain_event: GroundSceneDeployConfig,
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

    pub fn get_scene_setting( &self, biome_type: BiomeType ) -> &GroundSceneDeployConfig{
        match biome_type {
            BiomeType::Plain =>{ return &self.plain_event },
            /*
            BiomeType::Desert =>{},
            BiomeType::Forest =>{},
            BiomeType::Rocks =>{},
            BiomeType::Snow =>{},
            BiomeType::Swamp =>{},
            BiomeType::Tropic =>{},
            */
            _ => { return &self.plain_event },
        }
    }
}