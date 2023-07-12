use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use super::{stats::{Stat, ExtraStat}, skills::SkillType, abilities::Ability};
use crate::resources::scene_data::stuff::damage_type::DamageType;

//pub const PEREODIC_DAMAGE_TIME: f32 = 1000.0;

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
pub enum EffectTimeType {
    Pereodic,
    #[default]
    Instant,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct EffectDeploy {
    pub effect_type: EffectType,
    pub damage_type: DamageType,
    pub itself: bool,
    pub duration: u16,
    pub trigger_time: u16,

    pub extra_skill: Option<SkillType>,
    pub extra_skill_trigger: u8,
    
    // if +-10000 percent - we take value from weapon;
    pub change_stat: HashMap<Stat, i16>, // Stat and percentage
    pub change_stat_time_effect: EffectTimeType,
    pub change_stat_revert_changes: bool,

    pub change_extra_stat: HashMap<ExtraStat, i16>,
    pub change_extra_stat_time_effect: EffectTimeType,
    pub change_extra_stat_revert_changes: bool,
    pub change_extra_stat_is_damage: bool,

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
    pub trigger_time: f32,
    pub duration: f32,
    pub current_duration: f32,
    pub triggered: u16,

    pub extra_skill: Option<SkillType>,

    pub change_stat: HashMap<Stat, i16>, // Stat and flat damage to stat
    pub change_stat_time_effect: EffectTimeType,
    pub change_stat_revert_changes: bool,

    pub change_extra_stat: HashMap<ExtraStat, i16>,
    pub change_extra_stat_time_effect: EffectTimeType,
    pub change_extra_stat_revert_changes: bool,
    pub change_extra_stat_is_damage: bool,

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

impl Effect {
    pub fn new(config: &EffectDeploy) -> Self {
        Effect {
            effect_type: config.effect_type.clone(),
            damage_type: config.damage_type.clone(),
            trigger_time: config.trigger_time as f32 / 10.0,
            duration: config.duration as f32 / 10.0,
            current_duration: 0.0,
            triggered: 0,
            extra_skill: config.extra_skill.clone(),
            change_stat: config.change_stat.clone(),
            change_stat_time_effect: config.change_stat_time_effect.clone(),
            change_stat_revert_changes: config.change_stat_revert_changes,
            change_extra_stat: config.change_extra_stat.clone(),
            change_extra_stat_time_effect: config.change_extra_stat_time_effect.clone(),
            change_extra_stat_revert_changes: config.change_extra_stat_revert_changes,
            change_extra_stat_is_damage: config.change_extra_stat_is_damage,
            change_damage_resist: config.change_damage_resist.clone(),
            change_damage_resist_time_effect: config.change_damage_resist_time_effect.clone(),
            change_damage_resist_revert_changes: config.change_damage_resist_revert_changes,
            change_effect_resist: config.change_effect_resist.clone(),
            change_effect_resist_time_effect: config.change_effect_resist_time_effect.clone(),
            change_effect_resist_revert_changes: config.change_effect_resist_revert_changes,
            change_ability: config.change_ability.clone(),
            change_ability_time_effect: config.change_ability_time_effect.clone(),
            change_ability_revert_changes: config.change_ability_revert_changes,
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
