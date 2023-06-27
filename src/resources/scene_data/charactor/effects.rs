use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use super::{stats::Stat, resists::{DamageResistType, EffectResistType}, skills::PassiveSkill, damage_type::DamageType};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum EffectType{
    #[default]
    Stun,
    AcidEffect,
    AcidDamage,
    Moveless,
    Slow,
    BleedingEffect,
    BleedingDamage,
    BurnEffect,
    BurnDamage,
    ElectrificationEffect,
    ElectrificationDamage,
    FreezeEffect,
    FreezeDamage,
    Blind,
    PoisonEffect,
    PosionDamage,
    Wet,
    BrokeArmor,
    BrokeWeapon,
    RunFast,
    Lifelich,
    Staminalich,
}

pub enum EffectTargetType{
    Own,
    Enemy,
}

pub enum EffectTimeType {
    Pereodic,
    Instant,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct EffectDeploy {
    pub effect_type: EffectType,
    pub effect_time_type: EffectTimeType,
    pub damage_type: DamageType,
    pub target: EffectTargetType,
    // if 0 percents - we take value from weapon;
    pub change_stat: HashMap<Stat, i8>, // Stat and percentage
    pub change_damage_resist: HashMap<DamageResistType, i8>, // Damage Resist and percentage
    pub change_effect_resist: HashMap<EffectResistType, i8>, // Effect resist and percentage
    pub chnage_passive_skill: HashMap<PassiveSkill, i8>, // Passive Skill and percentage 
    pub duration: u8,
    pub revert_changes: bool,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Effect {
    pub effect_type: EffectType,
    pub damage_type: DamageType,
    pub change_stat: HashMap<Stat, i16>, //stat and flat value
    pub change_damage_resist: HashMap<DamageResistType, i16>,
    pub change_effect_resist: HashMap<EffectResistType, i16>,
    pub change_passive_skill: HashMap<PassiveSkill, i16>,
    pub duration: u16,
    pub current_duration: u16,
    pub revert_changes: bool,
}

pub fn get_effect_type_by_damage_type(damage_type: &DamageType) -> Vec<EffectType> {
    let mut vec: Vec<EffectType> = vec![];
    match *damage_type{
        DamageType::Acid => {
            vec.push(EffectType::AcidEffect);
            vec.push(EffectType::AcidDamage);
            vec.push(EffectType::BrokeArmor);
            vec.push(EffectType::BrokeWeapon);
        },
        DamageType::Cold => {
            vec.push(EffectType::Freeze);
            vec.push(EffectType::Slow);
        },
        DamageType::DeathEnegry => {
            vec.push(EffectType::Slow);
            vec.push(EffectType::Lifelich);
            vec.push(EffectType::Bleeding);
        },
        DamageType::Electric => {

        },
        DamageType::Fire => {

        },
        DamageType::Kinetic => {

        },
        DamageType::SacredEnergy => {

        },
        DamageType::Water => {

        }
    }
}
