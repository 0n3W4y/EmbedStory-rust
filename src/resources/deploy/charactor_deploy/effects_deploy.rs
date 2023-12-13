use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::resources::{scene_data::charactor::effects::{EffectType, EffectDeploy}, deploy::DEPLOY_EFFECTS_PATH};

#[derive(Deserialize, Debug)]
pub struct EffectsDeploy {
    stun: EffectDeploy,
    acid_damage: EffectDeploy,
    acid_debuff: EffectDeploy,
    moveless: EffectDeploy,
    bleeding_damage: EffectDeploy,
    bleeding_debuff: EffectDeploy,
    burn_damage: EffectDeploy,
    burn_debuff: EffectDeploy,
    cold_damage: EffectDeploy,
    cold_debuff: EffectDeploy,
    electric_damage: EffectDeploy,
    electric_debuff: EffectDeploy,
    water_damage: EffectDeploy,
    water_debuff: EffectDeploy,
    freeze: EffectDeploy,
    blind: EffectDeploy,
    poison_damage: EffectDeploy,
    posison_debuff: EffectDeploy,
    movement_buff: EffectDeploy,
    movement_debuff: EffectDeploy,
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
            EffectType::Stun => &self.stun,
            EffectType::Moveless => &self.moveless,
            EffectType::Freeze => &self.freeze,
            EffectType::Blind => &self.blind,
            EffectType::AcidDamage => &self.acid_damage,
            EffectType::AcidDebuff => &self.acid_debuff,
            EffectType::BleedingDamage => &self.bleeding_damage,
            EffectType::BleedingDebuff => &self.bleeding_debuff,
            EffectType::BurnDamage => &self.burn_damage,
            EffectType::BurnDebuff => &self.burn_debuff,
            EffectType::ColdDamage => &self.cold_damage,
            EffectType::ColdDebuff => &self.cold_debuff,
            EffectType::ElectricDamage => &self.electric_damage,
            EffectType::ElectricDebuff => &self.electric_debuff,
            EffectType::WaterDamage => &self.water_damage,
            EffectType::WaterDebuff => &self.water_debuff,
            EffectType::PoisonDamage => &self.poison_damage,
            EffectType::PosisonDebuff => &self.posison_debuff,
            EffectType::MovementBuff => &self.movement_buff,
            EffectType::MovementDebuff => &self.movement_debuff,
        }
    }
}