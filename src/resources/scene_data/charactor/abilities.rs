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
    CritChance,
    CritDamage,
    Accuracy,
    StaminaRegen,
    HealthRegen,
    ExperienceMultiplier,
    MeleeDamage,
    RangedDamage,
    MagicDamage,
    KineticDamage,
    FireDamage,
    ElectricDamage,
    WaterDamage,
    AcidDamage,
    PoisonDamage,
    ColdDamage,
    SacredEnergyDamage,
    DeathEnergyDamage,
    Moveless,
    Bleeding,
    Slow,
    Stun,
    Freeze,
    Burn,
    Electifical,
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
    let mut result: HashMap<AbilityType, f32> = HashMap::new();
    match *stat {
        Stat::Dexterity => {
            //RangedAttackDamage dex/6
            //TrinketRangedDamage dex/6
            let result_value = (value / 6) as f32;
            result.insert(AbilityType::RangedDamage, result_value);
        },
        Stat::Endurance => {
            //For regen END/100; 
            let result_value_hp_regen = (value as f32 / 100.0);
            
            change_ability(ability_storage, &AbilityType::HealthRegen, differece_for_regen);
        },
        Stat::Intellect => {
            //MagicWeaponDamage, INT/4
            //MagicTrinketDamage, INT/4
            let old_stat_value_for_magic_damage = (stat_value - changed_value) /4;
            let new_stat_value_for_magic_damage = stat_value / 4;
            let difference = new_stat_value_for_magic_damage - old_stat_value_for_magic_damage;
            change_ability(ability_storage, &AbilityType::MagicWeaponDamage, difference as f32);
            change_ability(ability_storage, &AbilityType::MagicTrinketDamage, difference as f32);
        },
        Stat::Luck => {
            //crit chanse LUCK * 1.5
            let old_stat_value_for_crit_chanse = (stat_value - changed_value) / 2;
            let new_stat_value_for_crit_chanse = stat_value / 2;
            let difference = new_stat_value_for_crit_chanse - old_stat_value_for_crit_chanse;
            change_ability(ability_storage, &AbilityType::CritChance, difference as f32);
        },
        Stat::Mobility => {
            //movement speed MOB*10;
            let old_value_for_move_speed = (stat_value - changed_value) * 10;
            let new_value_for_move_speed = stat_value * 10;
            let difference = new_value_for_move_speed - old_value_for_move_speed;
            change_ability(ability_storage, &AbilityType::MovementSpeed, difference as f32);

            //attackspeed mob*4;
            let old_stat_value_for_aspd = (stat_value - changed_value) * 5;
            let new_stat_value_for_aspd = stat_value *5;
            let difference_for_aspd = new_stat_value_for_aspd - old_stat_value_for_aspd;
            change_ability(ability_storage, &AbilityType::AttackSpeed, difference_for_aspd as f32);

            //evasion mob/4;
            let old_stat_value_for_evasion = (stat_value - changed_value) / 2;
            let new_stat_value_for_evasion = stat_value / 2;
            let differece_for_evasion = new_stat_value_for_evasion - old_stat_value_for_evasion;
            change_ability(ability_storage, &AbilityType::Evasion, differece_for_evasion as f32);
        },
        Stat::Strength => {
            //block amount STR/6;
            let old_value_for_block_amount = (stat_value - changed_value) / 6;
            let new_value_for_block_amount = stat_value / 6;
            let difference = new_value_for_block_amount - old_value_for_block_amount;
            change_ability(ability_storage, &AbilityType::BlockAmount, difference as f32);
            //meleeWeaponDamage STR/4
            //MeleeTrinketDamage STR/4
            let old_value_for_melee_damage = (stat_value - changed_value) / 4;
            let new_value_for_melee_damage = stat_value / 4;
            let difference_for_melee_damage = new_value_for_melee_damage - old_value_for_melee_damage;
            change_ability(ability_storage, &AbilityType::MeleeWeaponDamage, difference_for_melee_damage as f32);
            change_ability(ability_storage, &AbilityType::MeleeTrinketDamage, difference_for_melee_damage as f32);
        },
        Stat::Vitality => {
            //Stamina VIT *6;
            let old_value_for_stamina = (stat_value - changed_value) * 6;
            let new_value_for_stamina = stat_value * 6;
            let difference_for_stamina = new_value_for_stamina - old_value_for_stamina;
            change_extra_stat_cache(extra_stats_storage, extra_stats_cache, &ExtraStat::StaminaPoints, difference_for_stamina);

            //Stamina regen VIT /100
            let old_value_for_stamina_regen = (stat_value as f32 - changed_value as f32) / 100.0;
            let new_value_for_stamina_regen: f32 = stat_value as f32 / 100.0;
            let difference_for_stamina_regen = new_value_for_stamina_regen - old_value_for_stamina_regen;
            change_ability(ability_storage, &AbilityType::StaminaRegen, difference_for_stamina_regen);
        },
        Stat::Wisdom => {
            //ActiveSkillCD WIS * 10;
            let old_value_for_active_skill_cd = (stat_value - changed_value) * 10;
            let new_value_for_active_skill_cd = stat_value * 10;
            let difference = new_value_for_active_skill_cd - old_value_for_active_skill_cd;
            change_ability(ability_storage, &AbilityType::ActiveSkillsCoolDawn, difference as f32);
            //Crit multiplier WIS * 2;
            let old_value_for_crit_multiplier = (stat_value - changed_value) * 2;
            let new_value_for_crit_multiplier = stat_value * 2;
            let difference_ctir_multiplier = new_value_for_crit_multiplier - old_value_for_crit_multiplier;
            change_ability(ability_storage, &AbilityType::CritDamage, difference as f32);
        },
    }
    return result;
}