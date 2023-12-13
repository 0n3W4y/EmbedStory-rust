use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::resources::scene_data::{Stat, AbilityType, Attribute, ResistType};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum EffectType{
    #[default]
    Stun,
    AcidDamage,
    AcidDebuff,
    Moveless,
    BleedingDamage,
    BleedingDebuff,
    BurnDamage,
    BurnDebuff,
    ColdDamage,
    ColdDebuff,
    ElectricDamage,
    ElectricDebuff,
    WaterDamage,
    WaterDebuff,
    Freeze,
    Blind,
    PoisonDamage,
    PosisonDebuff,
    MovementBuff,
    MovementDebuff,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum EffectDamageType {
    Percent,
    #[default]
    Flat,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct EffectDeploy {
    pub effect_type: EffectType,
    pub effect_duration: f32,
    pub effect_trigger_time: f32,
    pub effect_trigger_chance: u8,
    
    pub change_stat: HashMap<Stat, i16>,
    pub change_attribute: HashMap<Attribute, i16>,
    pub change_attribute_cache: HashMap<Attribute, i16>,
    pub change_resist: HashMap<ResistType, i16>,
    pub change_ability: HashMap<AbilityType, i16>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Effect {
    pub effect_type: EffectType,
    pub effect_duration: f32,
    pub effect_trigger_time: f32,
    pub effect_trigger_chance: u8,
    pub current_time_duration: f32,
    pub total_time_duration: f32,

    pub change_stat: HashMap<Stat, i16>,
    pub change_attribute: HashMap<Attribute, i16>,
    pub change_attribute_cache: HashMap<Attribute, i16>,
    pub change_resist: HashMap<ResistType, i16>,
    pub change_ability: HashMap<AbilityType, i16>,
}

impl Effect {
    pub fn new(config: &EffectDeploy) -> Self {
        Effect {
            effect_type: config.effect_type.clone(),
            change_stat: config.change_stat.clone(),
            change_attribute: config.change_attribute.clone(),
            change_resist: config.change_resist.clone(),
            change_ability: config.change_ability.clone(),
            effect_duration: config.effect_duration,
            effect_trigger_time: config.effect_trigger_time,
            effect_trigger_chance: config.effect_trigger_chance,
            current_time_duration: 0.0,
            total_time_duration: 0.0,
            change_attribute_cache: config.change_attribute_cache.clone(),
        }
    }
}

/*
pub fn get_effect_type_by_damage_type(damage_type: &DamageType) -> Vec<EffectType> {
    match *damage_type{
        DamageType::Acid => vec![
            EffectType::Acid,
            EffectType::BrokeArmor,
            EffectType::BrokeWeapon
        ],
        DamageType::Cold => vec![
            EffectType::Freeze,
            EffectType::StaminaDamage
        ],
        DamageType::DeathEnegry => vec![
            EffectType::Slow,
            EffectType::Lifelich,
            EffectType::Bleeding
        ],
        DamageType::Electric => vec![
            EffectType::Electrification,
            //EffectType::Electroshoke,
            EffectType::StaminaDamage
        ],
        DamageType::Fire => vec![
            EffectType::Burn,
            EffectType::BrokeWeapon,
            EffectType::BrokeArmor
        ],
        DamageType::Phisical => vec![
            EffectType::Stun,
            EffectType::Moveless,
            EffectType::Slow,
            EffectType::Bleeding,
            EffectType::Blind,
            EffectType::StaminaDamage
        ],
        DamageType::Water => vec![
            EffectType::Bleeding,
            //EffectType::,
            EffectType::Slow
        ],
        DamageType::Poison => vec![

        ],
    }
}
*/
