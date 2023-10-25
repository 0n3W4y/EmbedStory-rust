use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use super::{stats::Stat, abilities::AbilityType};
use crate::resources::scene_data::stuff::damage_type::DamageType;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum EffectType{
    #[default]
    Stun,
    Acid,
    Moveless,
    Slow,
    Bleeding,
    Burn,
    Electrification,
    //Electroshoke,
    Freeze,
    Blind,
    Poison,
    Wet,
    BrokeArmor,
    BrokeWeapon,
    IncreaseMovement,
    Frostbite,
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
    pub duration: u16,
    
    pub change_stat: HashMap<Stat, i16>, // Stat and percentage
    pub change_stat_damage_type: EffectDamageType,
    pub change_damage_resist: HashMap<DamageType, i16>, // Damage Resist and percentage
    pub change_effect_resist: HashMap<EffectType, i16>, // Effect resist and percentage
    pub change_ability: HashMap<AbilityType, i16>, // Passive Skill and percentage 
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Effect {
    pub effect_type: EffectType,
    pub duration: f32,
    pub current_duration: f32,

    pub change_stat: HashMap<Stat, i16>, // Stat and flat damage to stat
    pub change_stat_damage_type: EffectDamageType,
    pub change_damage_resist: HashMap<DamageType, i16>, // Damage Resist and flat damage to resist
    pub change_effect_resist: HashMap<EffectType, i16>, // Effect resist and flat damage to resist
    pub change_ability: HashMap<AbilityType, i16>, // Passive Skill and flat damage to skill
}

impl Effect {
    pub fn new(config: &EffectDeploy) -> Self {
        Effect {
            effect_type: config.effect_type.clone(),
            duration: config.duration as f32 / 10.0,
            current_duration: 0.0,
            change_stat: config.change_stat.clone(),
            change_damage_resist: config.change_damage_resist.clone(),
            change_effect_resist: config.change_effect_resist.clone(),
            change_ability: config.change_ability.clone(),
            change_stat_damage_type: config.change_stat_damage_type.clone(),
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
