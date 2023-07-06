use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::abilities::Ability;
use self::effects::Effect;
use self::skills::{Skill, SkillType};
use self::stats::{ExtraStat, Stat};
use crate::resources::scene_data::charactor::effects::EffectType;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::stuff::damage_type::DamageType;
use super::stuff::Stuff;

pub mod abilities;
pub mod cleanup;
pub mod draw;
pub mod effects;
pub mod killed_charactor_handler;
pub mod move_charactor;
pub mod player_click_function;
pub mod skills;
pub mod stats;
pub mod update_effects;

pub const STATS_EVERY_LEVEL: u8 = 2;
pub const STATS_MIN_VALUE: u8 = 1;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum CharactorType {
    Player,
    NPC,
    #[default]
    Monster,
    Companion,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum GenderType {
    Female,
    #[default]
    Male,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum AttitudeToPlayer {
    #[default]
    Enemy,
    Friendly,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum RaceType {
    #[default]
    Human,
    Elf,
    Orc,
    Dwarf,
    Halfling,
    Undead,
    Naga,
    Gnome,
    Goblin,
    Beast,
    Arahnid,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Hash)]
pub enum StuffWearSlot {
    Head,
    Vest,
    Pants,
    Gloves,
    Shoes,
    Weapon,
    Trinket,
    Artifact,
    LeftRing,
    RightRing,
    Amulet,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
pub enum CharactorStatus {
    Dead,
    Moving,
    #[default]
    Standing,
    Attacking,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Charactor {
    pub id: usize,
    pub charactor_type: CharactorType,
    pub race_type: RaceType,
    pub gender_type: GenderType,

    pub status: CharactorStatus,

    //pub fraction: CharactorFraction,
    pub position: Position<i32>,
    pub destination_point: Position<i32>,
    pub destination_path: Vec<Position<i32>>,
    pub destination_direction: Position<i8>,

    pub level: u8,
    pub experience: u32,

    pub stats: HashMap<Stat, i16>,
    pub stats_cache: HashMap<Stat, i16>,

    pub extra_stats: HashMap<ExtraStat, i16>,
    pub extra_stats_cache: HashMap<ExtraStat, i16>,
    pub extra_stats_regen: HashMap<ExtraStat, f32>,

    pub damage_resists: HashMap<DamageType, i16>,

    pub effect_resits: HashMap<EffectType, i16>,

    pub ability: HashMap<Ability, f32>,

    pub skills: HashMap<u8, Option<Skill>>,
    pub passive_skills: HashMap<SkillType, Skill>,

    pub stuff_storage: Vec<Stuff>,
    pub stuff_storage_max_slots: u8,
    pub stuff_wear: HashMap<StuffWearSlot, usize>, // value is - stuff id;

    pub temporary_effect: HashMap<EffectType, Effect>,
    pub endless_effect: HashMap<EffectType, Effect>,
}

pub fn change_ability(
    ability_storage: &mut HashMap<Ability, f32>, 
    ability: &Ability,
    value: f32
) {
    ability_storage
        .entry(ability.clone())
        .and_modify(|old_value| *old_value += value)
        .or_insert(value);
}

pub fn change_effect_resist(
    effect_resists: &mut HashMap<EffectType, i16>,
    effect_resist: &EffectType,
    value: i16,
) {
    // if key is not in storage, we are added it to;
    effect_resists
        .entry(effect_resist.clone())
        .and_modify(|old_value| *old_value += value)
        .or_insert(value);
}

pub fn change_damage_resist(
    damage_resists: &mut HashMap<DamageType, i16>,
    damage_resist: &DamageType,
    value: i16,
) {
    // if key is not in storage, we are added it to;
    damage_resists
        .entry(damage_resist.clone())
        .and_modify(|old_value| *old_value += value)
        .or_insert(value);
}

pub fn change_extra_stat_cache(
    extra_stats_storage: &mut HashMap<ExtraStat, i16>,
    extra_stats_cache: &mut HashMap<ExtraStat, i16>,
    extra_stat: &ExtraStat,
    value: i16,
) {
    let old_value = match extra_stats_storage.get_mut(extra_stat) {
        Some(v) => v,
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. I created new entry with value:'{:?}' in stat STORAGE", 
                extra_stat,
                value
            );
            extra_stats_storage.insert(extra_stat.clone(), value);
            extra_stats_storage.get_mut(extra_stat).unwrap()
        }
    };

    let old_cahce_value = match extra_stats_cache.get_mut(extra_stat) {
        Some(v) => v,
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. I created new entry with value:'{:?}' in stat CACHE", 
                extra_stat,
                value
            );
            extra_stats_cache.insert(extra_stat.clone(), value);
            extra_stats_cache.get_mut(extra_stat).unwrap()
        }
    };

    let new_value = *old_cahce_value + value;

    if new_value <= 0 {
        *old_cahce_value = 1;
        *old_value = 1;
    } else if new_value < *old_cahce_value{
        *old_cahce_value = new_value;
        if *old_value + value < 0 {
            *old_value = 1;
        } else {
            *old_value += value;
        }
    } else {
        *old_cahce_value = value;
    }


}

pub fn change_extra_stat_current(
    extra_stats_storage: &mut HashMap<ExtraStat, i16>,
    extra_stats_cache: &mut HashMap<ExtraStat, i16>,
    extra_stat: &ExtraStat,
    value: i16,
){
    let cache_value = match extra_stats_cache.get(extra_stat) {
        Some(v) => *v,
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. I created new entry with value:'{:?}' in stat CACHE", 
                extra_stat,
                value
            );
            extra_stats_cache.insert(extra_stat.clone(), value);
            value
        }
    };

    let stat_value = match extra_stats_storage.get_mut(extra_stat) {
        Some(v) => v,
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. I created new entry with value:'{:?}' in stat STORAGE", 
                extra_stat,
                value
            );
            extra_stats_storage.insert(extra_stat.clone(), value);
            extra_stats_storage.get_mut(extra_stat).unwrap()
        }
    };

    let new_value = *stat_value + value;
    if new_value >= cache_value {
        *stat_value = cache_value;
    } else {
        *stat_value = new_value;
    }


}

pub fn change_extra_stat_by_regen(
    extra_stats_storage: &mut HashMap<ExtraStat, i16>,
    extra_stats_cache: &mut HashMap<ExtraStat, i16>,
    extra_stats_regen: &mut HashMap<ExtraStat, f32>,
    extra_stat: &ExtraStat,
    value: f32,
){
    let new_regen_value = match extra_stats_regen.get_mut(extra_stat) {
        Some(v) => {*v += value; v},
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. I created new entry with value:'{:?}' in stat REGEN", 
                extra_stat,
                value
            );
            extra_stats_regen.insert(extra_stat.clone(), value);
            extra_stats_regen.get_mut(extra_stat).unwrap()
        }
    };

    let new_value = new_regen_value.floor() as i16;
    if new_value > 0 {
        change_extra_stat_current(extra_stats_storage, extra_stats_cache, extra_stat, new_value);
    }
}

pub fn change_stat(
    stats_storage: &mut HashMap<Stat, i16>,
    stats_cache: &mut HashMap<Stat, i16>,
    extra_stats_regen: &mut HashMap<ExtraStat, f32>,
    extra_stats_storage: &mut HashMap<ExtraStat, i16>,
    extra_stats_cache: &mut HashMap<ExtraStat, i16>,
    effect_resists: &mut HashMap<EffectType, i16>,
    damage_resists: &mut HashMap<DamageType, i16>,
    ability_storage: &mut HashMap<Ability, i16>,
    stat: &Stat,
    value: i16,
    stats_min_value: u8,
) {
    match stats_cache.get_mut(stat) {
        Some(v) => *v += value,
        _ => {
            println!(
                "Can not change stat: '{:?}', because stat is not in storage. I create new entry with value: '{:?}'",
                stat,
                value
            );
            stats_cache.insert(stat.clone(), value);
        }
    };

    let stat_value = stats_cache.get(stat).unwrap(); //safe;

    let new_value = if *stat_value < stats_min_value as i16 {
        stats_min_value as i16
    } else {
        *stat_value
    };

    stats_storage
        .entry(stat.clone())
        .and_modify(|old_value| *old_value = new_value as i16)
        .or_insert(new_value as i16);

    do_stat_dependences(
        extra_stats_storage,
        extra_stats_cache,
        extra_stats_regen,
        effect_resists,
        damage_resists,
        ability_storage,
        stat,
        new_value,
        value,
    );
}

pub fn get_level_by_current_experience(experience: u32) -> u8 {
    let level: u8 = (experience as f64).sqrt() / 6.0;
}

pub fn do_stat_dependences(
    extra_stats_storage: &mut HashMap<ExtraStat, i16>,
    extra_stats_cache: &mut HashMap<ExtraStat, i16>,
    extra_stats_regen: &mut HashMap<ExtraStat, f32>,
    effect_resists: &mut HashMap<EffectType, i16>,
    damage_resists: &mut HashMap<DamageType, i16>,
    ability_storage: &mut HashMap<Ability, i16>,
    stat: &Stat,
    stat_value: i16,
    changed_value: i16,
) {
    match *stat {
        Stat::Dexterity => {
            //RangedAttackDamage dex/6
            //trinketRangedDamage dex/6
            let old_stat_value_for_trinket_damage = (stat_value - changed_value) / 4;
            let new_stat_value_for_trinket_damage = stat_value / 4;
            let difference = new_stat_value_for_trinket_damage - old_stat_value_for_trinket_damage;
            change_ability(ability_storage, &Ability::RangedTrinketDamage, difference);
            change_ability(ability_storage, &Ability::RangedWeaponDamage, difference);

            //attackspeed dex*4;
            let old_stat_value_for_aspd = (stat_value - changed_value) * 5;
            let new_stat_value_for_aspd = stat_value *5;
            let difference_for_aspd = new_stat_value_for_aspd - old_stat_value_for_aspd;
            change_ability(ability_storage, &Ability::AttackSpeed, difference_for_aspd);

            //evasion dex/4;
            let old_stat_value_for_evasion = (stat_value - changed_value) / 2;
            let new_stat_value_for_evasion = stat_value / 2;
            let differece_for_evasion = new_stat_value_for_evasion - old_stat_value_for_evasion;
            change_ability(ability_storage, &Ability::Evasion, differece_for_evasion);
        },
        Stat::Endurance => {
            //for HealthPoints END * 5; 
            let old_stat_value_for_healthpoints = (stat_value - changed_value) * 5;
            let new_stat_value_for_healthpoints = stat_value * 5;
            let difference = new_stat_value_for_healthpoints - old_stat_value_for_healthpoints;
            change_extra_stat_cache(extra_stats_storage, extra_stats_cache, &ExtraStat::HealthPoints, difference);
            //For regen END/100;
            let old_stat_value_for_regen = (stat_value as f32 - changed_value as f32) / 100.0;
            let new_stat_value_for_regen = stat_value as f32 / 100.0;
            let differece_for_regen = new_stat_value_for_regen - old_stat_value_for_regen;
            change_ability(ability_storage, &Ability::HealthRegen, differece_for_regen);

        },
        Stat::Intellect => {
            //MagicWeaponDamage, INT/4
            //MagicTrinketDamage, INT/4
            let old_stat_value_for_magic_damage = (stat_value - changed_value) /4;
            let new_stat_value_for_magic_damage = stat_value / 4;
            let difference = new_stat_value_for_magic_damage - old_stat_value_for_magic_damage;
            change_ability(ability_storage, &Ability::MagicWeaponDamage, difference);
            change_ability(ability_storage, &Ability::MagicTrinketDamage, difference);
        },
        Stat::Luck => {
            //crit chanse LUCK * 1.5
            let old_stat_value_for_crit_chanse = (stat_value - changed_value) / 2;
            let new_stat_value_for_crit_chanse = stat_value / 2;
            let difference = new_stat_value_for_crit_chanse - old_stat_value_for_crit_chanse;
            change_ability(ability_storage, &Ability::CritChance, difference);
        },
        Stat::Mobility => {
            //movement speed MOB*10;
            let old_value_for_move_speed = (stat_value - changed_value) * 10;
            let new_value_for_move_speed = stat_value * 10;
            let difference = new_value_for_move_speed - old_value_for_move_speed;
            change_ability(ability_storage, &Ability::MovementSpeed, difference);
        },
        Stat::Strength => {
            //block amount STR/6;
            let old_value_for_block_amount = (stat_value - changed_value) / 6;
            let new_value_for_block_amount = stat_value / 6;
            let difference = new_value_for_block_amount - old_value_for_block_amount;
            change_ability(ability_storage, &Ability::BlockAmount, difference);
            //meleeWeaponDamage STR/4
            //MeleeTrinketDamage STR/4
            let old_value_for_melee_damage = (stat_value - changed_value) / 4;
            let new_value_for_melee_damage = stat_value / 4;
            let difference_for_melee_damage = new_value_for_melee_damage - old_value_for_melee_damage;
            change_ability(ability_storage, &Ability::MeleeWeaponDamage, difference_for_melee_damage);
            change_ability(ability_storage, &Ability::MeleeTrinketDamage, difference_for_melee_damage);
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
            change_ability(ability_storage, &Ability::StaminaRegen, difference_for_stamina_regen);
        },
        Stat::Wisdom => {
            //ActiveSkillCD WIN * 10;
            let old_value_for_active_skill_cd = (stat_value - changed_value) * 10;
            let new_value_for_active_skill_cd = stat_value * 10;
            let difference = new_value_for_active_skill_cd - old_value_for_active_skill_cd;
            change_ability(ability_storage, &Ability::ActiveSkillsCoolDawn, difference);
        },
    }
}
