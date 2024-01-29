use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::resources::{scene_data::charactor::effects::{EffectType, EffectDeploy, OverTimeEffectDeploy, BuffDebuffEffectDeploy, OverTimeEffectType, BuffDebuffEffectType}, deploy::DEPLOY_EFFECTS_PATH};

#[derive(Deserialize, Debug)]
pub struct EffectsDeploy {
    primary_effects: PrimaryEffects,
    over_time_effects: OverTimeEffects,
    buff_debuff_effects: BuffDebuffEffects,
}

#[derive(Deserialize, Debug)]
pub struct PrimaryEffects {
    stun: EffectDeploy,
    moveless: EffectDeploy,
    freeze: EffectDeploy,
    blind: EffectDeploy,
    burn: EffectDeploy,
    acid: EffectDeploy,
    bleeding: EffectDeploy,
    cold: EffectDeploy,
    electroshocke: EffectDeploy,
    wet: EffectDeploy,
    regeneration: EffectDeploy,
    cheerfullness: EffectDeploy,
    myopia: EffectDeploy,
}

#[derive(Deserialize, Debug)]
pub struct OverTimeEffects {
    acid_damage: OverTimeEffectDeploy,
    bleed_damage: OverTimeEffectDeploy,
    cold_damage: OverTimeEffectDeploy,
    fire_damage: OverTimeEffectDeploy,
    electric_damage: OverTimeEffectDeploy,
    water_damage: OverTimeEffectDeploy,
    poison_damage: OverTimeEffectDeploy,
    stamina_damage: OverTimeEffectDeploy,
    health_damage: OverTimeEffectDeploy,
    health_regen: OverTimeEffectDeploy,
    stamina_regen: OverTimeEffectDeploy,
    none: OverTimeEffectDeploy,
}

#[derive(Deserialize, Debug)]
pub struct BuffDebuffEffects {
    fire_debuff: BuffDebuffEffectDeploy,
    cold_debuff: BuffDebuffEffectDeploy,
    acid_debuff: BuffDebuffEffectDeploy,
    bleed_debuff: BuffDebuffEffectDeploy,
    electric_debuff: BuffDebuffEffectDeploy,
    water_debuff: BuffDebuffEffectDeploy,
    poison_debuff: BuffDebuffEffectDeploy,
    stamina_debuff: BuffDebuffEffectDeploy,
    health_debuff: BuffDebuffEffectDeploy,
    stamina_buff: BuffDebuffEffectDeploy,
    health_buff: BuffDebuffEffectDeploy,
    accuracy_debuff: BuffDebuffEffectDeploy,
    none: BuffDebuffEffectDeploy,
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
            EffectType::Stun => &self.primary_effects.stun,
            EffectType::Moveless => &self.primary_effects.moveless,
            EffectType::Freeze => &self.primary_effects.freeze,
            EffectType::Blind => &self.primary_effects.blind,
            EffectType::Burn => &self.primary_effects.burn,
            EffectType::Acid => &self.primary_effects.acid,
            EffectType::Bleeding => &self.primary_effects.bleeding,
            EffectType::Cold => &self.primary_effects.cold,
            EffectType::Electroshocke => &self.primary_effects.electroshocke,
            EffectType::Wet => &self.primary_effects.wet,
            EffectType::Regeneration => &self.primary_effects.regeneration,
            EffectType::Cheerfullness => &self.primary_effects.cheerfullness,
            EffectType::Myopia => &self.primary_effects.myopia,
        }
    }

    pub fn get_over_time_effect_config (&self, effect_type: &OverTimeEffectType) -> &OverTimeEffectDeploy {
        match *effect_type {
            OverTimeEffectType::AcidDamage => &self.over_time_effects.acid_damage,
            OverTimeEffectType::BleedDamage => &self.over_time_effects.bleed_damage,
            OverTimeEffectType::ColdDamage => &self.over_time_effects.cold_damage,
            OverTimeEffectType::FireDamage => &self.over_time_effects.fire_damage,
            OverTimeEffectType::ElectricDamage => &self.over_time_effects.electric_damage,
            OverTimeEffectType::WaterDamage => &self.over_time_effects.water_damage,
            OverTimeEffectType::PoisonDamage => &self.over_time_effects.poison_damage,
            OverTimeEffectType::StaminaDamage => &self.over_time_effects.stamina_damage,
            OverTimeEffectType::HealthDamage => &self.over_time_effects.health_damage,
            OverTimeEffectType::HealthRegen => &self.over_time_effects.health_regen,
            OverTimeEffectType::StaminaRegen => &self.over_time_effects.stamina_regen,
            OverTimeEffectType::None =>  &self.over_time_effects.none,
        }
    }

    pub fn get_buff_debuff_effect_config (&self, effect_type: &BuffDebuffEffectType) -> &BuffDebuffEffectDeploy {
        match *effect_type {
            BuffDebuffEffectType::AcidDebuff => &self.buff_debuff_effects.acid_debuff,
            BuffDebuffEffectType::BleedDebuff => &self.buff_debuff_effects.bleed_debuff,
            BuffDebuffEffectType::ColdDebuff => &self.buff_debuff_effects.cold_debuff,
            BuffDebuffEffectType::FireDebuff => &self.buff_debuff_effects.fire_debuff,
            BuffDebuffEffectType::ElectricDebuff => &self.buff_debuff_effects.electric_debuff,
            BuffDebuffEffectType::WaterDebuff => &self.buff_debuff_effects.water_debuff,
            BuffDebuffEffectType::PoisionDebuff => &self.buff_debuff_effects.poison_debuff,
            BuffDebuffEffectType::StaminaDebuff => &self.buff_debuff_effects.stamina_debuff,
            BuffDebuffEffectType::HealthDebuff => &self.buff_debuff_effects.health_debuff,
            BuffDebuffEffectType::StaminaBuff => &self.buff_debuff_effects.stamina_buff,
            BuffDebuffEffectType::HealthBuff => &self.buff_debuff_effects.health_buff,
            BuffDebuffEffectType::AccuracyDebuff => &self.buff_debuff_effects.accuracy_debuff,
            BuffDebuffEffectType::None => &self.buff_debuff_effects.none,
        }
    }
}