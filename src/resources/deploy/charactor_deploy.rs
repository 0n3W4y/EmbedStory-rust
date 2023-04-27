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
    pub fn new() -> Self {
        let race_config_deploy: &str = "deploy/race_config.json";
        let race_config: RaceDeploy = match File::open(race_config_deploy){
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                serde_json::from_str(&content).expect("JSON was not well-formatted")
            },
            Err(e) => panic!("Can not open objects data file: {}, {}", e, race_config_deploy),
        };

        return CharactorDeploy{
            race_config
        };
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
    pub resist_max_value: i16,
    pub resist_min_value: i16,
    pub stat_extra_points: u8,
    pub stat_min_value: u8,
    pub body_structure: HashMap<BodyPartType, i16>,
    pub body_structure_part_type: PartType,
}


