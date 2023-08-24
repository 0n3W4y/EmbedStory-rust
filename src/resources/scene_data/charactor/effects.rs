use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use super::{stats::{Stat, ExtraStat}, skills::SkillType, abilities::AbilityType};
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
    Lifelich,
    Staminalich,
    StaminaDamage,
    HealthRegen,
    StaminaRegen,
    Frostbite,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum StatDamageType {
    Percent,
    #[default]
    Flat,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct EffectDeploy {
    pub effect_type: EffectType,
    pub duration: u16,
    pub trigger_time: u16,

    pub extra_skill: HashMap<SkillType, u8>, // extra skill ( passive ) and trigger percent;
    
    pub change_stat: HashMap<Stat, i16>, // Stat and percentage
    pub change_stat_damage_type: StatDamageType,

    pub change_extra_stat: HashMap<ExtraStat, i16>,
    pub change_extra_stat_damage_type: StatDamageType,

    pub change_damage_resist: HashMap<DamageType, i16>, // Damage Resist and percentage
    pub change_damage_resist_damage_type: StatDamageType,

    pub change_effect_resist: HashMap<EffectType, i16>, // Effect resist and percentage
    pub change_effect_resist_damage_type: StatDamageType,

    pub change_ability: HashMap<AbilityType, i16>, // Passive Skill and percentage 
    pub change_ability_damage_type: StatDamageType,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Effect {
    pub effect_type: EffectType,
    pub trigger_time: f32,
    pub duration: f32,
    pub current_duration: f32,
    pub total_duration: f32,

    pub extra_skill: HashMap<SkillType, u8>,

    pub change_stat: HashMap<Stat, i16>, // Stat and flat damage to stat
    pub change_stat_damage_type: StatDamageType,

    pub change_extra_stat: HashMap<ExtraStat, i16>,
    pub change_extra_stat_damage_type: StatDamageType,

    pub change_damage_resist: HashMap<DamageType, i16>, // Damage Resist and flat damage to resist
    pub change_damage_resist_damage_type: StatDamageType,

    pub change_effect_resist: HashMap<EffectType, i16>, // Effect resist and flat damage to resist
    pub change_effect_resist_damage_type: StatDamageType,

    pub change_ability: HashMap<AbilityType, i16>, // Passive Skill and flat damage to skill
    pub change_ability_damage_type: StatDamageType,
}

impl Effect {
    pub fn new(config: &EffectDeploy) -> Self {
        Effect {
            effect_type: config.effect_type.clone(),
            trigger_time: config.trigger_time as f32 / 10.0,
            duration: config.duration as f32 / 10.0,
            current_duration: 0.0,
            total_duration: 0.0,
            extra_skill: config.extra_skill.clone(),
            change_stat: config.change_stat.clone(),
            change_extra_stat: config.change_extra_stat.clone(),
            change_damage_resist: config.change_damage_resist.clone(),
            change_effect_resist: config.change_effect_resist.clone(),
            change_ability: config.change_ability.clone(),
            change_stat_damage_type: config.change_stat_damage_type.clone(),
            change_extra_stat_damage_type: config.change_extra_stat_damage_type.clone(),
            change_damage_resist_damage_type: config.change_damage_resist_damage_type.clone(),
            change_effect_resist_damage_type: config.change_effect_resist_damage_type.clone(),
            change_ability_damage_type: config.change_ability_damage_type.clone(),
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
        DamageType::Kinetic => vec![
            EffectType::Stun,
            EffectType::Moveless,
            EffectType::Slow,
            EffectType::Bleeding,
            EffectType::Blind,
            EffectType::StaminaDamage
        ],
        DamageType::SacredEnergy => vec![
            EffectType::Blind,
            EffectType::Staminalich,
            EffectType::Stun
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
