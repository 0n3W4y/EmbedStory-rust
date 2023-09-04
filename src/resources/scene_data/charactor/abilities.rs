use std::collections::HashMap;

use serde::Deserialize;

use super::stats::Stat;

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