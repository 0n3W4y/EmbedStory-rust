use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use crate::resources::scene_data::objects::body_part::PartType;
use crate::resources::scene_data::objects::body_part::BodyPartType;
use crate::resources::scene_data::objects::charactor::RaceType;
use crate::resources::scene_data::objects::resists::Resist;

#[derive( Deserialize, Debug)]
pub struct CharactorDeploy{
    pub race_config: RaceDeploy,
}

impl CharactorDeploy{
    pub fn new(path: &str) -> Self {
        let result: CharactorDeploy  = match File::open( path ){
            Ok( mut file ) => {
                let mut contents = String::new();
                file.read_to_string( &mut contents ).unwrap();
                serde_json::from_str( &contents ).expect( "JSON was not well-formatted" )
            }
            Err( err ) => panic!( "Can not open objects data file: {}, {}", err, path ),
        };

        return result;
    }

    pub fn get_race_config(&self, race_type: &RaceType) -> &RaceConfig{
        match race_type {
            RaceType::Human => &self.race_config.human,
            RaceType::Humanoid => &self.race_config.humanoid,
            RaceType::Mutant => &self.race_config.mutant,
            RaceType::Robot => &self.race_config.robot,
            RaceType::SuperMutant => &self.race_config.super_mutant
        }
    }
}


#[derive(Deserialize, Debug)]
pub struct RaceDeploy{
    pub human: RaceConfig,
    pub humanoid: RaceConfig,
    pub mutant: RaceConfig,
    pub robot: RaceConfig,
    pub super_mutant: RaceConfig,

}

#[derive(Deserialize, Debug)]
pub struct RaceConfig{
    pub resists: HashMap<Resist, i16>,
    pub stats_extra_point: u8,
    pub body_structure: HashMap<BodyPartType, u16>,
    pub body_structure_part_type: PartType,
}


