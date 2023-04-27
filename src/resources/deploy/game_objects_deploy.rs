use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

use crate::resources::scene_data::objects::thing::{ThingConfig, ThingType};

#[derive( Deserialize, Debug )]
pub struct GameObjectsDeploy{
    pub rock: ThingConfig,
    pub boulder: ThingConfig,
    pub tree: ThingConfig,
    pub fertile_tree: ThingConfig,
    pub bush: ThingConfig,
    pub fertile_bush: ThingConfig,
    pub log: ThingConfig,
    pub copper_ore: ThingConfig,
    pub iron_ore: ThingConfig,
    pub wooden_wall: ThingConfig,
    pub stone_wall: ThingConfig,
    pub iron_wall: ThingConfig,
    pub steel_wall: ThingConfig,
    pub wooden_door: ThingConfig,
    pub reinforced_wooden_door: ThingConfig,
    pub iron_door: ThingConfig,
    pub reinforced_iron_door: ThingConfig,
    pub steel_door: ThingConfig,
    pub reinforced_steel_door: ThingConfig,
}
impl GameObjectsDeploy{
    pub fn new( path: &str ) -> Self{
        let result: GameObjectsDeploy  = match File::open( path ){
            Ok( mut file ) => {
                let mut contents = String::new();
                file.read_to_string( &mut contents ).unwrap();
                serde_json::from_str( &contents ).expect( "JSON was not well-formatted" )
            }
            Err( err ) => panic!( "Can not open objects data file: {}, {}", err, path ),
        };

        return result;
    }

    pub fn get_config( &self, thing_type: &ThingType ) -> &ThingConfig{
        return match thing_type {
            ThingType::Rock => { &self.rock },
            _ => { &self.rock }
        }
    }
}