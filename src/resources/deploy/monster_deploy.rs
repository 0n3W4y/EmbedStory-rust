use std::{fs::File, io::Read, collections::HashMap};

use serde::Deserialize;

use crate::resources::scene_data::{AbilityType, Stat, Attribute, ResistType};

use super::{DEPLOY_MONSTER_STRENGTH_PATH, DEPLOY_MONSTER_TYPE_PATH};

#[derive(Deserialize, Debug)]
pub struct MonsterDeploy {
    monster_strength: MonsterStrengthDeploy,
    monster_type: MonsterTypeDeploy,
}

impl MonsterDeploy {
    pub fn new() -> Self {
        let monster_strength: MonsterStrengthDeploy = match File::open(DEPLOY_MONSTER_STRENGTH_PATH) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).expect("JSON was not well-formatted")
            }
            Err(err) => panic!("Can not open ground scene data file: {}, {}", err, DEPLOY_MONSTER_STRENGTH_PATH),
        };

        let monster_type: MonsterTypeDeploy = match File::open(DEPLOY_MONSTER_TYPE_PATH) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).expect("JSON was not well-formatted")
            }
            Err(err) => panic!("Can not open ground scene data file: {}, {}", err, DEPLOY_MONSTER_TYPE_PATH),
        };

        MonsterDeploy {  
            monster_strength,
            monster_type,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct MonsterStrengthDeploy {
    weak: MonsterStrengthConfig,
    normal: MonsterStrengthConfig,
    champion: MonsterStrengthConfig,
    elite: MonsterStrengthConfig,
    boss: MonsterStrengthConfig,
}

#[derive(Deserialize, Debug)]
pub struct MonsterStrengthConfig {
    pub stats: i16,
    pub attributes: i16,
    pub resists: i16,
    pub abilities: HashMap<AbilityType, i16>,
}

#[derive(Deserialize, Debug)]
pub struct MonsterTypeDeploy {
    pub melee: MonsterTypeConfig,
    pub ranged: MonsterTypeConfig,
    pub magic: MonsterTypeConfig,

}

#[derive(Deserialize, Debug)] 
pub struct MonsterTypeConfig{
    pub stats: HashMap<Stat, i16>,
    pub attributes: HashMap<Attribute, i16>,
    pub resists: HashMap<ResistType, i16>,
    pub abilities: HashMap<AbilityType, i16>,
}