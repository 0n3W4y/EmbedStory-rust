use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::resources::{scene_data::charactor::effects::{EffectType, EffectDeploy}, deploy::DEPLOY_EFFECTS_PATH};

#[derive(Deserialize, Debug)]
pub struct EffectsDeploy {
    stun: EffectDeploy,
    acid_debuff: EffectDeploy,
    moveless: EffectDeploy,
    bleeding_debuff: EffectDeploy,
    fire_debuff: EffectDeploy,
    cold_debuff: EffectDeploy,
    electric_debuff: EffectDeploy,
    water_debuff: EffectDeploy,
    freeze: EffectDeploy,
    blind: EffectDeploy,
    poison_debuff: EffectDeploy,
    movement_buff: EffectDeploy,
    movement_debuff: EffectDeploy,
    acid_damage: EffectDeploy,
    bleeding_damage: EffectDeploy,
    cold_damage: EffectDeploy,
    water_damage: EffectDeploy,
    electric_damage: EffectDeploy,
    fire_damage: EffectDeploy,
    poison_damage: EffectDeploy,
    stamina_damage: EffectDeploy,
    health_regen: EffectDeploy,
    stamina_regen: EffectDeploy,
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
            EffectType::AcidDebuff => &self.acid_debuff,
            EffectType::BleedingDebuff => &self.bleeding_debuff,
            EffectType::FireDebuff => &self.fire_debuff,
            EffectType::ColdDebuff => &self.cold_debuff,
            EffectType::ElectricDebuff => &self.electric_debuff,
            EffectType::WaterDebuff => &self.water_debuff,
            EffectType::PoisonDebuff => &self.poison_debuff,
            EffectType::MovementBuff => &self.movement_buff,
            EffectType::MovementDebuff => &self.movement_debuff,
            EffectType::AcidDamage => &self.acid_damage,
            EffectType::BleedingDamage => &self.bleeding_damage,
            EffectType::ColdDamage => &self.cold_damage,
            EffectType::FireDamage => &self.fire_damage,
            EffectType::ElectricDamage => &self.electric_damage,
            EffectType::WaterDamage => &self.water_damage,
            EffectType::PoisonDamage => &self.poison_damage,
            EffectType::StaminaDamage => &self.stamina_damage,
            EffectType::StaminaRegen => &self.stamina_regen,
            EffectType::HealthRegen => &self.health_regen,
        }
    }
}