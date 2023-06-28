use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use super::{stats::{Stat, ExtraStat}, resists::{DamageResistType, EffectResistType}, skills::PassiveSkill};
use crate::resources::scene_data::stuff::damage_type::DamageType;

pub const PEREODIC_DAMAGE_TIME: f32 = 1000.0;

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
    RunFast,
    Lifelich,
    Staminalich,
    StaminaDamage,
}

pub enum EffectTimeType {
    Pereodic,
    Instant,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct EffectDeploy {
    pub effect_type: EffectType,
    pub damage_type: DamageType,
    pub itself: bool,
    pub duration: u8,
    
    // if +-101 percent - we take value from weapon;
    pub change_stat: HashMap<Stat, i8>, // Stat and percentage
    pub change_stat_time_effect: EffectTimeType,
    pub change_stat_revert_changes: bool,

    pub change_extra_stat: HashMap<ExtraStat, i8>,
    pub change_extra_stat_time_effect: EffectTimeType,
    pub change_extra_stat_revert_changes: bool,

    pub change_damage_resist: HashMap<DamageResistType, i8>, // Damage Resist and percentage
    pub change_damage_resist_time_effect: EffectTimeType,
    pub change_damage_resist_revert_changes: bool,

    pub change_effect_resist: HashMap<EffectResistType, i8>, // Effect resist and percentage
    pub change_effect_resist_time_effect: EffectTimeType,
    pub change_effect_resist_revert_changes: bool,

    pub change_passive_skill: HashMap<PassiveSkill, i8>, // Passive Skill and percentage 
    pub change_passive_skill_time_effect: EffectTimeType,
    pub change_passive_skill_revert_changes: bool,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Effect {
    pub effect_type: EffectType,
    pub damage_type: DamageType,
    pub duration: u16,
    pub current_duration: u16,
    pub triggered: u8,

    pub change_stat: HashMap<Stat, i16>, // Stat and flat damage to stat
    pub change_stat_time_effect: EffectTimeType,
    pub change_stat_revert_changes: bool,

    pub change_extra_stat: HashMap<ExtraStat, i16>,
    pub change_extra_stat_time_effect: EffectTimeType,
    pub change_extra_stat_revert_changes: bool,

    pub change_damage_resist: HashMap<DamageResistType, i16>, // Damage Resist and flat damage to resist
    pub change_damage_resist_time_effect: EffectTimeType,
    pub change_damage_resist_revert_changes: bool,

    pub change_effect_resist: HashMap<EffectResistType, i16>, // Effect resist and flat damage to resist
    pub change_effect_resist_time_effect: EffectTimeType,
    pub change_effect_resist_revert_changes: bool,

    pub change_passive_skill: HashMap<PassiveSkill, i16>, // Passive Skill and flat damage to skill 
    pub change_passive_skill_time_effect: EffectTimeType,
    pub change_passive_skill_revert_changes: bool,
}

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
            EffectType::Lifelich,
            EffectType::Slow
        ]
    }
}
