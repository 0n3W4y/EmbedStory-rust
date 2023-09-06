use std::collections::HashMap;

use serde::Deserialize;

use crate::resources::scene_data::stuff::damage_type::DamageType;

use super::{stats::Stat, effects::EffectType, skills::SkillType};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Deserialize)]
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
    MeleeDamage,
    RangedDamage,
    MagicDamage,
    CuttingDamage,
    PiercingDamage,
    CrushingDamage,
    FireDamage,
    ElectricDamage,
    WaterDamage,
    AcidDamage,
    PoisonDamage,
    ColdDamage,
    Moveless,
    Bleeding,
    Slow,
    Stun,
    Freeze,
    Burn,
    Electrification,
    Blind,
    Acid,
    Poison,
    Wet,
    BrokeArmor,
    BrokeWeapon,
    StaminaDamage,
    Staminalich,
    Lifelich,
    StaminaRegenEffect,
    HealthRegenEffect,
    Frostbite,
    IncreseMovementSpeed,
    DecreaseMovementSpeed,
}

//formulas
pub fn get_values_of_abilities_from_stat(stat: &Stat, value: i16) -> HashMap<AbilityType, i16> {
    let mut result: HashMap<AbilityType, i16> = HashMap::new();
    match *stat {
        Stat::Dexterity => {
            //RangedAttackDamage dex/4
            let result_value = value / 4;
            result.insert(AbilityType::RangedDamage, result_value);
        },
        Stat::Endurance => {
            //For hp regen END/5; 
            let result_value_hp_regen = value / 5;            
            result.insert(AbilityType::HealthRegen, result_value_hp_regen);
            //block amount END/10;
            let result_value_block_amount = value / 10;
            result.insert(AbilityType::BlockAmount, result_value_block_amount);
        },
        Stat::Intellect => {
            //MagicWeaponDamage, INT/4
            let result_value_magic_damage = value / 4;
            result.insert(AbilityType::MagicDamage, result_value_magic_damage);
        },
        Stat::Luck => {
            //crit chanse = LUCK / 10
            let result_value = value / 10;
            result.insert(AbilityType::CriticalHitChanse, result_value);

            // crit multiplier = luck / 5;
            let result_value_crit_multiplier = value / 5;
            result.insert(AbilityType::CriticalHitMultiplier, result_value_crit_multiplier);
        },
        Stat::Mobility => {
            //movement speed MOB;
            let result_value_movement = value;
            result.insert(AbilityType::MovementSpeed, result_value_movement);

            //attackspeed = mob / 10;
            let result_value_attack_speed = value / 10;
            result.insert(AbilityType::AttackSpeed, result_value_attack_speed);

            //evasion mob/8;
            let result_value_evasion = value / 8;
            result.insert(AbilityType::Evasion, result_value_evasion);
        },
        Stat::Strength => {
            //MeleeDamage STR/4
            let result_value_melee_damage = value / 4;
            result.insert(AbilityType::MeleeDamage, result_value_melee_damage);
        },
        Stat::Vitality => {
            //for sp regen vit/5;
            let result_value_sp_regen = value / 5;
            result.insert(AbilityType::StaminaRegen, result_value_sp_regen);
        },
        Stat::Wisdom => {
            //ActiveSkillCD WIS * 10;
            let result_value_skill_cd = value / 10;
            result.insert(AbilityType::ActiveSkillsCoolDawn, result_value_skill_cd);
        },
    }
    return result;
}

pub fn get_effect_type_from_ability (ability_storage: &HashMap<AbilityType, i16>, effect_type: &EffectType) -> i16 {
    return match *effect_type {
        EffectType::Stun => match ability_storage.get(&AbilityType::Stun) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::Stun => match ability_storage.get(&AbilityType::Stun) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::Acid =>  match ability_storage.get(&AbilityType::Acid) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::Moveless => match ability_storage.get(&AbilityType::Moveless) {
            Some(v) => *v,
            None => 0,   
        },
        EffectType::Slow =>  match ability_storage.get(&AbilityType::Slow) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::Bleeding => match ability_storage.get(&AbilityType::Bleeding) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::Burn => match ability_storage.get(&AbilityType::Burn) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::Electrification => match ability_storage.get(&AbilityType::Electrification) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::Freeze => match ability_storage.get(&AbilityType::Freeze) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::Blind => match ability_storage.get(&AbilityType::Blind) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::Poison => match ability_storage.get(&AbilityType::Poison) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::Wet => match ability_storage.get(&AbilityType::Wet) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::BrokeArmor => match ability_storage.get(&AbilityType::BrokeArmor) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::BrokeWeapon => match ability_storage.get(&AbilityType::BrokeWeapon) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::IncreaseMovement => match ability_storage.get(&AbilityType::IncreseMovementSpeed) {
            Some(v) => *v,
            None => 0,
        },
        EffectType::Frostbite => match ability_storage.get(&AbilityType::Frostbite) {
            Some(v) => *v,
            None => 0,
        },
    }
}

pub fn get_damage_type_from_ability (ability_storage: &HashMap<AbilityType, i16>, damage_type: &DamageType) -> i16 {
    return match *damage_type {
        DamageType::Fire => {
            match ability_storage.get(&AbilityType::FireDamage) {
                Some(v) => *v,
                None => 0,
            }
        },
        DamageType::Cold => {
            match ability_storage.get(&AbilityType::ColdDamage) {
                Some(v) => *v,
                None => 0,
            }
        },
        DamageType::Electric => {
            match ability_storage.get(&AbilityType::ElectricDamage) {
                Some(v) => *v,
                None => 0,
            }
        },
        DamageType::Cutting => {
            match ability_storage.get(&AbilityType::CuttingDamage) {
                Some(v) => *v,
                None => 0,
            }
        },
        DamageType::Piercing => {
            match ability_storage.get(&AbilityType::PiercingDamage) {
                Some(v) => *v,
                None => 0,
            }
        },
        DamageType::Crushing => {
            match ability_storage.get(&AbilityType::CrushingDamage) {
                Some(v) => *v,
                None => 0,
            }
        },
        DamageType::Water => {
                match ability_storage.get(&AbilityType::WaterDamage) {
                Some(v) => *v,
                None => 0,
            }
        },
        DamageType::Acid => {
            match ability_storage.get(&AbilityType::AcidDamage) {
                Some(v) => *v,
                None => 0,
            }
        },
        DamageType::Poison => {
            match ability_storage.get(&AbilityType::PoisonDamage) {
                Some(v) => *v,
                None => 0,
            }
        },
    }
}

pub fn get_skill_damage_from_ability (ability_storage: &HashMap<AbilityType, i16>, skill_type: &SkillType) -> i16 {
    match *skill_type {
        SkillType::Melee => match ability_storage.get(&AbilityType::MeleeDamage) {
            Some(v) => *v,
            None => 0,
        },
        SkillType::Ranged => match ability_storage.get(&AbilityType::RangedDamage) {
            Some(v) => *v,
            None => 0,
        },
        SkillType::Magic => match ability_storage.get(&AbilityType::MagicDamage) {
            Some(v) => *v,
            None => 0,
        },
    }
}