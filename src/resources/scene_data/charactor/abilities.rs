use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::resources::scene_data::stuff::damage_type::DamageType;

use super::stats::Stat;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Deserialize, Serialize)]
pub enum AbilityType{
    Evasion,
    #[default]
    MovementSpeed,
    AttackSpeed,
    ActiveSkillsCoolDawn,
    BlockChance,
    BlockAmount,
    CriticalHitChanse,
    CriticalHitMultiplier,
    Accuracy,
    StaminaRegen,
    HealthRegen,
    ExperienceMultiplier,
    PhisicalDamage,
    FireDamage,
    ElectricDamage,
    WaterDamage,
    AcidDamage,
    PoisonDamage,
    ColdDamage,
    HealthDamage,
    StaminaDamage,
}

impl AbilityType {
    pub fn all_values() -> impl Iterator<Item = Self> {
        vec![
            AbilityType::Evasion,
            AbilityType::MovementSpeed,
            AbilityType::AttackSpeed,
            AbilityType::ActiveSkillsCoolDawn,
            AbilityType::BlockAmount,
            AbilityType::BlockChance,
            AbilityType::CriticalHitChanse,
            AbilityType::CriticalHitMultiplier,
            AbilityType::Accuracy,
            AbilityType::StaminaRegen,
            AbilityType::HealthRegen,
            AbilityType::ExperienceMultiplier,
            AbilityType::PhisicalDamage,
            AbilityType::FireDamage,
            AbilityType::ElectricDamage,
            AbilityType::WaterDamage,
            AbilityType::AcidDamage,
            AbilityType::PoisonDamage,
            AbilityType::ColdDamage,
            AbilityType::HealthDamage,
            AbilityType::StaminaDamage,
        ].into_iter()
    }
}

//formulas
pub fn get_values_of_abilities_from_stat(stat: &Stat, value: i16) -> HashMap<AbilityType, i16> {
    let mut result: HashMap<AbilityType, i16> = HashMap::new();
    match *stat {
        Stat::Dexterity => {
            let evasion = value / 15;             //evasion:  dex/15
            let movement_speed = value / 10;      //move speed: dex/10;
            let attack_speed = value / 10;        //atk speed: dex/10;
            result.insert(AbilityType::Evasion, evasion);
            result.insert(AbilityType::MovementSpeed, movement_speed);
            result.insert(AbilityType::AttackSpeed, attack_speed);
        },
        Stat::Intellect => {
            let cooldown_active_skill = value / 10;           //cd of active skills: INT /10;
            let critical_multiplier = value / 2;              // Crit Multi : INT / 5;
            let stamina_regen = value / 10;             //stamina regen: INT / 10
            result.insert(AbilityType::ActiveSkillsCoolDawn, cooldown_active_skill);
            result.insert(AbilityType::CriticalHitMultiplier, critical_multiplier);
            result.insert(AbilityType::StaminaRegen, stamina_regen);
        },
        Stat::Luck => {
            let critical_hit_chance = value / 10;               //crit chance: LCK / 10;
            let block_chance = value / 10;                      //block chance: LCK /10;
            let accuracy = value / 10;                      //accuracy: LCK / 10;
            result.insert(AbilityType::CriticalHitChanse, critical_hit_chance);
            result.insert(AbilityType::BlockChance, block_chance);
            result.insert(AbilityType::Accuracy, accuracy);
        },
        Stat::Strength => {
            let health_regen = value / 10;              //health reneg: STR / 10;
            let block_amount = value / 10;              //block amount: STR / 10;
            result.insert(AbilityType::HealthRegen, health_regen);
            result.insert(AbilityType::BlockAmount, block_amount);
        },
        _ => {},
    }
    return result;
}

pub fn get_ability_type_from_damage_type (damage_type: &DamageType) -> AbilityType {
    return match *damage_type {
        DamageType::Fire => AbilityType::FireDamage,
        DamageType::Cold => AbilityType::ColdDamage,
        DamageType::Electric => AbilityType::ElectricDamage,
        DamageType::Phisical => AbilityType::PhisicalDamage,
        DamageType::Acid => AbilityType::AcidDamage,
        DamageType::Poison => AbilityType::PoisonDamage,
        DamageType::Water => AbilityType::WaterDamage,
        DamageType::Health => AbilityType::HealthDamage,
        DamageType::Stamina => AbilityType::StaminaDamage,
        
    }
}