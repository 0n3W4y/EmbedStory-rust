use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::resources::scene_data::{Stat, AbilityType, Attribute, ResistType, stuff::damage_type::DamageType};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum EffectType{
    Burn,
    Acid,
    Bleeding,
    Cold,
    Electroshocke,
    Wet,
    Stun,
    Moveless,
    Freeze,
    Blind,
    Regeneration,
    Cheerfullness,
    Myopia,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum OverTimeEffectType {
    AcidDamage,
    BleedDamage,
    ColdDamage,
    FireDamage,
    ElectricDamage,
    WaterDamage,
    PoisonDamage,
    StaminaDamage,
    HealthDamage,
    HealthRegen,
    StaminaRegen,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum BuffDebuffEffectType {
    AcidDebuff,
    BleedDebuff,
    ColdDebuff,
    FireDebuff,
    ElectricDebuff,
    WaterDebuff,
    PoisionDebuff,
    StaminaDebuff,
    HealthDebuff,
    StaminaBuff,
    HealthBuff,
    AccuracyDebuff,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum EffectStatus {
    CanNotMove,
    CanNotAttack,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EffectDeploy {
    pub effect_type: EffectType,
    pub effect_lifetime: f32,

    pub over_time_effect: OverTimeEffectDeploy,
    pub buff_debuff_effect: BuffDebuffEffectDeploy,
    pub effect_status: Vec<EffectStatus>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OverTimeEffectDeploy {
    pub effect_type: OverTimeEffectType,
    pub effect_damage_type: DamageType,
    pub trigger_time_effect: f32,
    pub change_attributes: HashMap<Attribute, i16>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BuffDebuffEffectDeploy {
    pub effect_type: BuffDebuffEffectType,
    pub change_stat: HashMap<Stat, i16>,
    pub change_attribute_cache: HashMap<Attribute, i16>,
    pub change_resist: HashMap<ResistType, i16>,
    pub change_ability: HashMap<AbilityType, i16>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OverTimeEffect {
    pub effect_type: OverTimeEffectType,
    pub effect_damage_type: DamageType,
    pub trigger_time_effect: f32,
    pub time_duration: f32,
    pub change_attributes: HashMap<Attribute, i16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuffDebuffEffect {
    pub effect_type: BuffDebuffEffectType,
    pub change_stat: HashMap<Stat, i16>,
    pub change_attribute_cache: HashMap<Attribute, i16>,
    pub change_resist: HashMap<ResistType, i16>,
    pub change_ability: HashMap<AbilityType, i16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effect {
    pub effect_type: EffectType,
    pub effect_lifetime: f32,
    pub time_duration: f32,

    pub over_time_effect: Option<OverTimeEffect>,
    pub buff_debuff_effect: Option<BuffDebuffEffect>,
    pub effect_status: Vec<EffectStatus>,
}

impl Effect {
    pub fn new(config: &EffectDeploy) -> Self {
        let over_time_effect = if config.over_time_effect.effect_type == OverTimeEffectType::None {
            None
        } else {
            Some(
                OverTimeEffect {
                    effect_type: config.over_time_effect.effect_type.clone(),
                    effect_damage_type: config.over_time_effect.effect_damage_type.clone(),
                    trigger_time_effect: config.over_time_effect.trigger_time_effect,
                    time_duration: 0.0,
                    change_attributes: config.over_time_effect.change_attributes.clone(),
                }
            )
        };

        let buff_debuff_effect = if config.buff_debuff_effect.effect_type == BuffDebuffEffectType::None {
            None
        } else {
            Some(
                BuffDebuffEffect {
                    effect_type: config.buff_debuff_effect.effect_type.clone(),
                    change_stat: config.buff_debuff_effect.change_stat.clone(),
                    change_attribute_cache: config.buff_debuff_effect.change_attribute_cache.clone(),
                    change_resist: config.buff_debuff_effect.change_resist.clone(),
                    change_ability: config.buff_debuff_effect.change_ability.clone(),
                }
            )
        };

        Effect {
            effect_type: config.effect_type.clone(),
            effect_lifetime: config.effect_lifetime,
            time_duration: 0.0,
            over_time_effect,
            buff_debuff_effect,
            effect_status: config.effect_status.to_vec(),
        }
    }
}
