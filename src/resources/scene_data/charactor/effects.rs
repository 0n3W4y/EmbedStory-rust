use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::resources::{scene_data::{Stat, Ability, Attribute, Resist, Damage}, deploy::Deploy};

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
    Poison,
    Restoration,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum OverTimeEffectType {
    AcidDamage,
    BleedDamage,
    ColdDamage,
    FireDamage,
    ElectricDamage,
    WaterDamage,
    PoisonDamageHealth,
    PoisonDamageStamina,
    StaminaDamage,
    HealthDamage,
    #[default]
    HealthRegen,
    StaminaRegen,
    HealthRestoration,
    StaminaRestoration,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum BuffDebuffEffectType {
    AcidDebuff,
    BleedDebuff,
    ColdDebuff,
    FireDebuff,
    ElectricDebuff,
    WaterDebuff,
    PoisonDebuff,
    StaminaDebuff,
    HealthDebuff,
    StaminaBuff,
    #[default]
    HealthBuff,
    AccuracyDebuff,
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

    pub over_time_effect: Vec<OverTimeEffectType>,
    pub buff_debuff_effect: Vec<BuffDebuffEffectType>,
    pub effect_status: Vec<EffectStatus>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct OverTimeEffectDeploy {
    pub effect_type: OverTimeEffectType,
    pub effect_damage_type: Damage,
    pub trigger_time_effect: f32,
    pub effect_damage_value: i16,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct BuffDebuffEffectDeploy {
    pub effect_type: BuffDebuffEffectType,
    pub change_stat: HashMap<Stat, i16>,
    pub change_attribute_cache: HashMap<Attribute, i16>,
    pub change_resist: HashMap<Resist, i16>,
    pub change_ability: HashMap<Ability, i16>,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OverTimeEffect {
    pub effect_type: OverTimeEffectType,
    pub effect_damage_type: Damage,
    pub trigger_time_effect: f32,
    pub time_duration: f32,
    pub effect_damage_value: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BuffDebuffEffect {
    pub effect_type: BuffDebuffEffectType,
    pub change_stat: HashMap<Stat, i16>,
    pub change_attribute_cache: HashMap<Attribute, i16>,
    pub change_resist: HashMap<Resist, i16>,
    pub change_ability: HashMap<Ability, i16>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Effect {
    pub effect_type: EffectType,
    pub effect_lifetime: f32,
    pub time_duration: f32,

    pub over_time_effect: Vec<OverTimeEffect>,
    pub buff_debuff_effect: Vec<BuffDebuffEffect>,
    pub effect_status: Vec<EffectStatus>,
}

impl Effect {
    pub fn new(deploy: &Deploy, effect_type: &EffectType) -> Self {
        let effect_config = deploy.charactor_deploy.effects_deploy.get_effect_config(effect_type);
        let mut over_time_effect: Vec<OverTimeEffect> = vec![];
        for over_time_effect_type in effect_config.over_time_effect.iter() {
            let over_time_effect_config = deploy.charactor_deploy.effects_deploy.get_over_time_effect_config(over_time_effect_type);
            let new_over_time_effect = OverTimeEffect {
                effect_type: over_time_effect_config.effect_type.clone(),
                effect_damage_type: over_time_effect_config.effect_damage_type.clone(),
                trigger_time_effect: over_time_effect_config.trigger_time_effect,
                time_duration: 0.0,
                effect_damage_value: over_time_effect_config.effect_damage_value,
            };
            over_time_effect.push(new_over_time_effect);
        } 

        let mut buff_debuff_effect: Vec<BuffDebuffEffect> = vec![];
        for buff_debuff_effect_type in effect_config.buff_debuff_effect.iter() {
            let buff_debuff_effect_config = deploy.charactor_deploy.effects_deploy.get_buff_debuff_effect_config(buff_debuff_effect_type);
            let new_buff_debuff_effect = BuffDebuffEffect {
                effect_type: buff_debuff_effect_config.effect_type.clone(),
                change_stat: buff_debuff_effect_config.change_stat.clone(),
                change_attribute_cache: buff_debuff_effect_config.change_attribute_cache.clone(),
                change_resist: buff_debuff_effect_config.change_resist.clone(),
                change_ability: buff_debuff_effect_config.change_ability.clone(),
            };
            buff_debuff_effect.push(new_buff_debuff_effect);
        }

        Effect {
            effect_type: effect_config.effect_type.clone(),
            effect_lifetime: effect_config.effect_lifetime,
            time_duration: 0.0,
            over_time_effect,
            buff_debuff_effect,
            effect_status: effect_config.effect_status.to_vec(),
        }
    }
}
