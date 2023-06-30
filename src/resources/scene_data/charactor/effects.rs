use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use super::{stats::{Stat, ExtraStat}, skills::Skill, abilities::Ability};
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
    pub duration: u16,

    pub extra_skill: Option<Skill>,
    pub extra_skill_trigger: u8,
    
    // if +-10000 percent - we take value from weapon;
    pub change_stat: HashMap<Stat, i16>, // Stat and percentage
    pub change_stat_time_effect: EffectTimeType,
    pub change_stat_revert_changes: bool,

    pub change_extra_stat: HashMap<ExtraStat, i16>,
    pub change_extra_stat_time_effect: EffectTimeType,
    pub change_extra_stat_revert_changes: bool,

    pub change_damage_resist: HashMap<DamageType, i16>, // Damage Resist and percentage
    pub change_damage_resist_time_effect: EffectTimeType,
    pub change_damage_resist_revert_changes: bool,

    pub change_effect_resist: HashMap<EffectType, i16>, // Effect resist and percentage
    pub change_effect_resist_time_effect: EffectTimeType,
    pub change_effect_resist_revert_changes: bool,

    pub change_ability: HashMap<Ability, i16>, // Passive Skill and percentage 
    pub change_ability_time_effect: EffectTimeType,
    pub change_ability_revert_changes: bool,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Effect {
    pub effect_type: EffectType,
    pub damage_type: DamageType,
    pub duration: u16,
    pub current_duration: u16,
    pub triggered: u16,

    pub extra_skill: Option<Skill>,
    pub extra_skill_trigger: u8,

    pub change_stat: HashMap<Stat, i16>, // Stat and flat damage to stat
    pub change_stat_time_effect: EffectTimeType,
    pub change_stat_revert_changes: bool,

    pub change_extra_stat: HashMap<ExtraStat, i16>,
    pub change_extra_stat_time_effect: EffectTimeType,
    pub change_extra_stat_revert_changes: bool,

    pub change_damage_resist: HashMap<DamageType, i16>, // Damage Resist and flat damage to resist
    pub change_damage_resist_time_effect: EffectTimeType,
    pub change_damage_resist_revert_changes: bool,

    pub change_effect_resist: HashMap<EffectType, i16>, // Effect resist and flat damage to resist
    pub change_effect_resist_time_effect: EffectTimeType,
    pub change_effect_resist_revert_changes: bool,

    pub change_ability: HashMap<Ability, i16>, // Passive Skill and flat damage to skill 
    pub change_ability_time_effect: EffectTimeType,
    pub change_ability_revert_changes: bool,
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
