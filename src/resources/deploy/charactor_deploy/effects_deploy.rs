use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::resources::{scene_data::charactor::effects::{EffectType, EffectDeploy}, deploy::DEPLOY_EFFECTS_PATH};

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
    pub fn new() -> Self {
        let effects_deploy: EffectsDeploy = match File::open(DEPLOY_EFFECTS_PATH) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                serde_json::from_str(&content).expect("JSON was not well-formatted")
            }
            Err(e) => panic!("Can not open objects data file: {}, {}", e, DEPLOY_EFFECTS_PATH),
        };
        return effects_deploy;
    }
    
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