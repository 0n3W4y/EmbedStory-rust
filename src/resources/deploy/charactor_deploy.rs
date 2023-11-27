use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::resources::scene_data::{Stat, Attribute, AbilityType, ResistType};
use crate::resources::scene_data::charactor::effects::{EffectDeploy, EffectType};
use crate::resources::scene_data::charactor::skills::{SkillDeploy, SkillType};
use crate::resources::scene_data::charactor::RaceType;

#[derive(Deserialize, Debug)]
pub struct CharactorDeploy {
    pub race_deploy: RaceDeploy,
    pub effects_deploy: EffectsDeploy,
    pub skills_deploy: SkillsDeploy,
}

impl CharactorDeploy {
    pub fn new() -> Self {
        let race_config_deploy: &str = "deploy/race_config.json";
        let effects_config: &str = "deploy/battle_effects_config.json";
        let skills_config: &str = "deploy/skills_config.json";

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

        let skills_deploy: SkillsDeploy = match File::open(skills_config) {
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
            skills_deploy,
        };
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
    frostbite: EffectDeploy,
    increase_movement_speed: EffectDeploy,
}

impl EffectsDeploy {
    pub fn get_effect_config(&self, effect_type: &EffectType) -> &EffectDeploy {
        match *effect_type {
            EffectType::Acid => &self.acid,
            EffectType::Stun => &self.stun,
            EffectType::Moveless => &self.moveless,
            EffectType::Slow => &self.slow,
            EffectType::Bleeding => &self.bleeding,
            EffectType::Burn => &self.burn,
            EffectType::Electrification => &self.electrification,
            EffectType::Freeze => &self.freeze,
            EffectType::Blind => &self.blind,
            EffectType::Poison => &self.poison,
            EffectType::Wet => &self.wet,
            EffectType::BrokeArmor => &self.broke_armor,
            EffectType::BrokeWeapon => &self.broke_weapon,
            EffectType::IncreaseMovement => &self.increase_movement_speed,
            EffectType::Frostbite => &self.frostbite,
        }
    }
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

impl RaceDeploy {
    pub fn get_race_config(&self, race_type: &RaceType) -> &RaceConfig {
        match race_type {
            RaceType::Human => &self.human,
            RaceType::Elf => &self.elf,
            RaceType::Orc => &self.orc,
            RaceType::Halfling => &self.halfling,
            RaceType::Dwarf => &self.dwarf,
            RaceType::Undead => &self.undead,
            RaceType::Naga => &self.naga,
            RaceType::Gnome => &self.gnome,
            RaceType::Goblin => &self.goblin,
            RaceType::Beast => &self.beast,
            RaceType::Arahnid => &self.arahnid,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RaceConfig {
    pub stats: HashMap<Stat, i16>,
    pub attributes: HashMap<Attribute, i16>,
    pub resists: HashMap<ResistType, i16>,
    pub ability: HashMap<AbilityType, i16>,
    pub passive_skills: Vec<SkillType>,
}

#[derive(Deserialize, Debug)]
pub struct SkillsDeploy{
    pub base_skill: SkillDeploy,
}

impl SkillsDeploy {
    pub fn get_skill_deploy( &self, skill: &SkillType ) -> &SkillDeploy {
        match *skill {
            SkillType::BaseSkill => &self.base_skill,
        }
    }
}


