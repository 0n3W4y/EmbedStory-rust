use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::resources::scene_data::charactor::abilities::Ability;
use crate::resources::scene_data::charactor::effects::{EffectDeploy, EffectType};
use crate::resources::scene_data::charactor::skills::PassiveSkill;
use crate::resources::scene_data::charactor::stats::{ExtraStat, Stat};
use crate::resources::scene_data::charactor::RaceType;
use crate::resources::scene_data::stuff::damage_type::DamageType;

#[derive(Deserialize, Debug)]
pub struct CharactorDeploy {
    pub race_deploy: RaceDeploy,
    pub effects_deploy: EffectsDeploy,
}

impl CharactorDeploy {
    pub fn new() -> Self {
        let race_config_deploy: &str = "deploy/race_config.json";
        let effects_config: &str = "deploy/battle_effects_config.json";

        let race_deploy: RaceDeploy = match File::open(race_config_deploy) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                serde_json::from_str(&content).expect("JSON was not well-formatted")
            }
            Err(e) => panic!(
                "Can not open objects data file: {}, {}",
                e, race_config_deploy
            ),
        };

        let effects_deploy: EffectsDeploy = match File::open(effects_config) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                serde_json::from_str(&content).expect("JSON was not well-formatted")
            }
            Err(e) => panic!("Can not open objects data file: {}, {}", e, effects_config),
        };

        return CharactorDeploy {
            race_deploy,
            effects_deploy,
        };
    }

    pub fn get_race_config(&self, race_type: &RaceType) -> &RaceConfig {
        match race_type {
            RaceType::Human => &self.race_deploy.human,
            RaceType::Elf => &self.race_deploy.elf,
            RaceType::Orc => &self.race_deploy.orc,
            RaceType::Halfling => &self.race_deploy.halfling,
            RaceType::Dwarf => &self.race_deploy.dwarf,
            RaceType::Undead => &self.race_deploy.undead,
            RaceType::Naga => &self.race_deploy.naga,
            RaceType::Gnome => &self.race_deploy.gnome,
            RaceType::Goblin => &self.race_deploy.goblin,
            RaceType::Beast => &self.race_deploy.beast,
            RaceType::Arahnid => &self.race_deploy.arahnid,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct EffectsDeploy {
    stun: EffectDeploy,
    acid: EffectDeploy,
    moveless: EffectDeploy,
    slow: EffectDeploy,
    bleeding: EffectDeploy,
    burn: EffectDeploy,
    electrification: EffectDeploy,
    freeze: EffectDeploy,
    blind: EffectDeploy,
    poison: EffectDeploy,
    wet: EffectDeploy,
    broke_armor: EffectDeploy,
    broke_weapon: EffectDeploy,
    run_fast: EffectDeploy,
    lifelich: EffectDeploy,
    staminalich: EffectDeploy,
    stamina_damage: EffectDeploy,
    healthpoint_regen: EffectDeploy,
    stamina_regen: EffectDeploy,
}

#[derive(Deserialize, Debug)]
pub struct RaceDeploy {
    pub human: RaceConfig,
    pub elf: RaceConfig,
    pub orc: RaceConfig,
    pub halfling: RaceConfig,
    pub dwarf: RaceConfig,
    pub undead: RaceConfig,
    pub naga: RaceConfig,
    pub gnome: RaceConfig,
    pub goblin: RaceConfig,
    pub beast: RaceConfig,
    pub arahnid: RaceConfig,
}

#[derive(Deserialize, Debug)]
pub struct RaceConfig {
    pub stats: HashMap<Stat, i16>,
    pub stats_min_value: u8,
    pub extra_stats: HashMap<ExtraStat, i16>,
    pub damage_resists: HashMap<DamageType, i16>,
    pub damage_resists_min_value: i16,
    pub damage_resists_max_value: i16,
    pub effect_resists: HashMap<EffectType, i16>,
    pub effect_resist_min_value: i16,
    pub effect_resist_max_value: i16,
    pub ability: HashMap<Ability, i16>,
    pub passive_skills: HashMap<PassiveSkill, i16>,
    pub endless_effect: Vec<EffectType>,
}
