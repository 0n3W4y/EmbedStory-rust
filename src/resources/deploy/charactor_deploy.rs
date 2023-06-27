use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;



#[derive( Deserialize, Debug)]
pub struct CharactorDeploy{
    pub race_deploy: RaceDeploy,
}

impl CharactorDeploy{
    pub fn new() -> Self {
        let race_config_deploy: &str = "deploy/race_config.json";
        let charactor_subtype_config_deploy: &str = "deploy/charactor_subtype_config.json";



        let race_deploy: RaceDeploy = match File::open(race_config_deploy){
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                serde_json::from_str(&content).expect("JSON was not well-formatted")
            },
            Err(e) => panic!("Can not open objects data file: {}, {}", e, race_config_deploy),
        };

        let charactor_subtype_deploy: CharactorSubTypeDeploy = match File::open(charactor_subtype_config_deploy){
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                serde_json::from_str(&content).expect("JSON was not well-formatted")
            },
            Err(e) => panic!("Can not open objects data file: {}, {}", e, charactor_subtype_config_deploy),
        };


        return CharactorDeploy{
            race_deploy,
            charactor_subtype_deploy,
        };
    }

    pub fn get_race_config(&self, race_type: &RaceType) -> &RaceConfig{
        match race_type {
            RaceType::Human => &self.race_deploy.human,
            RaceType::Humanoid => &self.race_deploy.humanoid,
            RaceType::Mutant => &self.race_deploy.mutant,
            RaceType::Robot => &self.race_deploy.robot,
            RaceType::SuperMutant => &self.race_deploy.super_mutant,
            RaceType::Bogomol => &self.race_deploy.bogomol,
        }
    }

    pub fn get_charactor_subtype_config(&self, charactor_subtype: &CharactorSubType) -> &CharactorSubTypeConfig {
        match charactor_subtype {
            //TODO: Do all ;
            CharactorSubType::Civilian => &self.charactor_subtype_deploy.civilian,
            CharactorSubType::MeleeFighter => &self.charactor_subtype_deploy.melee_fighter,
            CharactorSubType::MixedFighter => &self.charactor_subtype_deploy.mixed_fighter,
            CharactorSubType::RangedFighter => &self.charactor_subtype_deploy.range_fighter,
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
    pub bogomol: RaceConfig,

}

#[derive(Deserialize, Debug)]
pub struct RaceConfig{
    pub resists: HashMap<Resist, i16>,
    pub resist_max_value: i16,
    pub resist_min_value: i16,
    pub stat_min_value: u8,
    pub conditions: HashMap<ConditionType, u16>,
    pub body_structure: HashMap<BodyPartType, i16>,
    pub body_structure_part_type: PartType,
}

#[derive(Deserialize, Debug)]
pub struct CharactorSubTypeDeploy {
    pub civilian: CharactorSubTypeConfig,
    pub melee_fighter: CharactorSubTypeConfig,
    pub range_fighter: CharactorSubTypeConfig,
    pub mixed_fighter: CharactorSubTypeConfig,
}

#[derive(Deserialize, Debug)]
pub struct CharactorSubTypeConfig {
    pub stats: HashMap<Stat, u8>,
    pub stat_max_random_value: i8,
}


