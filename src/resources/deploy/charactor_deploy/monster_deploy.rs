use std::{fs::File, io::Read, collections::HashMap};

use serde::Deserialize;

use crate::resources::{scene_data::{Ability, charactor::CharactorStrength}, deploy::DEPLOY_MONSTER_STRENGTH_PATH};



#[derive(Deserialize, Debug)]
pub struct MonsterDeploy {
    pub weak: MonsterConfig,
    pub normal: MonsterConfig,
    pub champion: MonsterConfig,
    pub elite: MonsterConfig,
    pub boss: MonsterConfig,
}

impl MonsterDeploy {
    pub fn new() -> Self {
        let monster_deploy: MonsterDeploy = match File::open(DEPLOY_MONSTER_STRENGTH_PATH) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).expect("JSON was not well-formatted")
            }
            Err(err) => panic!("Can not open ground scene data file: {}, {}", err, DEPLOY_MONSTER_STRENGTH_PATH),
        };

        return monster_deploy
    }

    pub fn get_monster_strength(&self, charactor_strength: &CharactorStrength) -> &MonsterConfig {
        match *charactor_strength {
            CharactorStrength::Weak => &self.weak,
            CharactorStrength::Normal => &self.normal,
            CharactorStrength::Champion => &self.champion,
            CharactorStrength::Elite => &self.elite,
            CharactorStrength::Boss => &self.boss,
            CharactorStrength::None => &self.normal,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct MonsterConfig {
    pub stats: i16,
    pub attributes: i16,
    pub resists: i16,
    pub abilities: HashMap<Ability, i16>,
}