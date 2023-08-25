use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::abilities::AbilityType;
use self::effects::{Effect, StatDamageType};
use self::skills::{Skill, SkillType};
use self::stats::{Stat, ExtraStat};
use crate::resources::scene_data::charactor::effects::EffectType;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::stuff::damage_type::DamageType;
use super::stuff::Stuff;

pub mod abilities;
pub mod cleanup;
pub mod draw;
pub mod effects;
pub mod killed_charactor_handler;
pub mod update_move;
pub mod player_click_function;
pub mod skills;
pub mod stats;
pub mod update_effects;
pub mod update_passive_skills;
pub mod update_attack;
pub mod update_cooldowns;

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
    PickupItem,
}

#[derive(Debug, Default)]
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

    pub damage_resists: HashMap<DamageType, i16>,

    pub effect_resits: HashMap<EffectType, i16>,

    pub ability: HashMap<AbilityType, i16>,

    pub skills: HashMap<u8, Skill>,
    pub passive_skills: HashMap<SkillType, Skill>,

    pub stuff_storage: Vec<Stuff>,
    pub stuff_storage_max_slots: u8,
    pub stuff_wear: HashMap<StuffWearSlot, Option<Stuff>>,

    pub temporary_effect: HashMap<EffectType, Effect>,
    pub endless_effect: HashMap<EffectType, Effect>,
}

pub fn change_ability(
    ability_storage: &mut HashMap<AbilityType, i16>, 
    ability: &AbilityType,
    value: i16
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
    stat_damage_type: &StatDamageType,
) {
    let default_extra_stat_value = 100;
    //chech for stat in storage;
    let stat_value = match extra_stats_storage.get_mut(extra_stat) {
        Some(v) => v,
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. I created new entry with value:'{:?}' in stat STORAGE", 
                extra_stat,
                default_extra_stat_value
            );
            extra_stats_storage.insert(extra_stat.clone(), default_extra_stat_value);
            extra_stats_storage.get_mut(extra_stat).unwrap()
        }
    };

    let old_stat_value = *stat_value;

    //check for stat in cache;
    let cache_value = match extra_stats_cache.get_mut(extra_stat) {
        Some(v) => v,
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. I created new entry with value:'{:?}' in stat CACHE", 
                extra_stat,
                default_extra_stat_value
            );
            extra_stats_cache.insert(extra_stat.clone(), default_extra_stat_value);
            extra_stats_cache.get_mut(extra_stat).unwrap()
        }
    };

    let old_cache_value = *cache_value;

    //calculating new cache value;
    let new_cache_value = if *stat_damage_type == StatDamageType::Flat {
        old_cache_value + value
    } else {
        old_cache_value + old_cache_value * value / 100
    };

    if new_cache_value <= 0 {
        println!("Extra stat cache value <= 0!");
    };

    //set new value to cache;
    *cache_value = new_cache_value;

    //calculating new stat value;
    let new_stat_value = if *stat_damage_type == StatDamageType::Flat {
        *stat_value + value
    } else {
        *stat_value + old_cache_value * value / 100
    };

    if new_stat_value < 1 {
        *stat_value = 1;
    } else {
        *stat_value = new_stat_value;
    }
}

pub fn change_extra_stat_current(
    extra_stats_storage: &mut HashMap<ExtraStat, i16>,
    extra_stats_cache: &mut HashMap<ExtraStat, i16>,
    extra_stat: &ExtraStat,
    value: i16,
    stat_damage_type: &StatDamageType,
){
    let default_extra_stat_value = 100;

    let cache_value = match extra_stats_cache.get(extra_stat) {
        Some(v) => *v,
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. I created new entry with value:'{:?}' in stat CACHE", 
                extra_stat,
                value
            );
            extra_stats_cache.insert(extra_stat.clone(), default_extra_stat_value);
            default_extra_stat_value
        }
    };

    let stat_value = match extra_stats_storage.get_mut(extra_stat) {
        Some(v) => v,
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. I created new entry with value:'{:?}' in stat STORAGE", 
                extra_stat,
                default_extra_stat_value
            );
            extra_stats_storage.insert(extra_stat.clone(), default_extra_stat_value);
            extra_stats_storage.get_mut(extra_stat).unwrap()
        }
    };

    let new_value: i16 = if *stat_damage_type == StatDamageType::Flat {
        *stat_value + value
    } else {
        *stat_value + *stat_value * value / 100
    };

    if new_value < cache_value {
        *stat_value = new_value;
    } else {
        *stat_value = cache_value;
    }
}

pub fn change_stat(
    stats_storage: &mut HashMap<Stat, i16>,
    stats_cache: &mut HashMap<Stat, i16>,
    extra_stats_storage: &mut HashMap<ExtraStat, i16>,
    extra_stats_cache: &mut HashMap<ExtraStat, i16>,
    effect_resists: &mut HashMap<EffectType, i16>,
    damage_resists: &mut HashMap<DamageType, i16>,
    ability_storage: &mut HashMap<AbilityType, i16>,
    stat: &Stat,
    value: i16,
    stat_damage_type: &StatDamageType,
    stats_min_value: u8,
) {
    //first check for stat in cache storage;
    let default_stat_value = 10;

    let cache_value = match stats_cache.get_mut(stat) {
        Some(v) => v,
        _ => {
            println!(
                "Can not change stat: '{:?}', because stat is not in storage. I create new entry with value: '{:?}'",
                stat,
                default_stat_value
            );
            stats_cache.insert(stat.clone(), default_stat_value);
            stats_cache.get_mut(stat).unwrap()
        }
    };

    //check stat value in storage
    let stat_value = match stats_storage.get_mut(stat) {
        Some(v) => v,
        None => {
            println!(
                "Can not change stat: '{:?}', because stat is not in storage. I create new entry with value: '{:?}'",
                stat,
                default_stat_value
            );
            stats_cache.insert(stat.clone(), default_stat_value);
            stats_storage.get_mut(stat).unwrap()
        },
    };

    //set value to old;
    let old_stat_value: i16 = *stat_value;

    //calculating values and set them to cache;
    if *stat_damage_type == StatDamageType::Flat {
        *cache_value += value;
    } else {
        *cache_value += *stat_value * value / 100;
    }

    //check stat to minimal value;
    if *cache_value < stats_min_value as i16{
        *stat_value = stats_min_value as i16;
    } else {
        *stat_value = *cache_value;
    }   

    //check for change dependences;
    if *stat_value == old_stat_value {
        //nothing to change;
        return;
    };

    do_stat_dependences(
        extra_stats_storage,
        extra_stats_cache,
        effect_resists,
        damage_resists,
        ability_storage,
        stat,
        *stat_value,
        old_stat_value,
    );
}

pub fn get_level_by_current_experience(experience: u32) -> u8 {
    ((experience as f64).sqrt() / 6.0) as u8
}

pub fn do_stat_dependences(
    extra_stats_storage: &mut HashMap<ExtraStat, i16>,
    extra_stats_cache: &mut HashMap<ExtraStat, i16>,
    effect_resists: &mut HashMap<EffectType, i16>,
    damage_resists: &mut HashMap<DamageType, i16>,
    ability_storage: &mut HashMap<AbilityType, i16>,
    stat: &Stat,
    new_value: i16,
    old_value: i16,
) {
    let old_values_for_abilities: HashMap<AbilityType, i16> = abilities::get_values_of_abilities_from_stat(stat, old_value);
    let new_values_for_abilities: HashMap<AbilityType, i16> = abilities::get_values_of_abilities_from_stat(stat, new_value);
    for (ability_type, value) in new_values_for_abilities.iter() {
        //from new value substruct old value and add brand new value to abiliti storage;
        let value_to_ability = value - old_values_for_abilities.get(ability_type).unwrap();
        change_ability(ability_storage, ability_type, value_to_ability);
    };

    let old_values_for_extra_stat: HashMap<ExtraStat, i16> = stats::get_values_of_extra_stats_from_stat(stat, old_value);
    let new_values_for_extra_stat: HashMap<ExtraStat, i16> = stats::get_values_of_extra_stats_from_stat(stat, new_value);
    for (extra_stat, value) in new_values_for_extra_stat.iter() {
        let value_to_extra_stat = value - old_values_for_extra_stat.get(extra_stat).unwrap();
        change_extra_stat_cache(extra_stats_storage, extra_stats_cache, extra_stat, value_to_extra_stat, &StatDamageType::Flat);
    };

    let old_values_for_damage_resist: HashMap<DamageType, i16> = get_values_of_damage_resists_from_stat(stat, old_value);
    let new_values_for_damage_resist: HashMap<DamageType, i16> = get_values_of_damage_resists_from_stat(stat, new_value);
    for (damage_resist, value) in new_values_for_damage_resist.iter() {
        let value_to_damage_resist = value - old_values_for_damage_resist.get(damage_resist).unwrap();
        change_damage_resist(damage_resists, damage_resist, value_to_damage_resist);
    };

    let old_values_for_effect_resist: HashMap<EffectType, i16> = get_values_of_effect_resists_from_stat(stat, old_value);
    let new_values_for_effect_resist: HashMap<EffectType, i16> = get_values_of_effect_resists_from_stat(stat, new_value);
    for (effect_resist, value) in new_values_for_effect_resist.iter() {
        let value_to_effect_resist = value - old_values_for_effect_resist.get(effect_resist).unwrap();
        change_effect_resist(effect_resists, effect_resist, value_to_effect_resist);
    };
}

//formulas
pub fn get_values_of_damage_resists_from_stat(stat: &Stat, value: i16) -> HashMap<DamageType, i16> {
    let mut result: HashMap<DamageType, i16> = HashMap::new();
    match *stat {
        Stat::Strength => {},
        Stat::Dexterity => {},
        Stat::Mobility => {},
        Stat::Wisdom => {},
        Stat::Intellect => {},
        Stat::Luck => {},
        Stat::Vitality => {
            //fire, cold, acid, electric, water vit/10;
            let result_value = value / 10;
            result.insert(DamageType::Acid, result_value);
            result.insert(DamageType::Fire, result_value);
            result.insert(DamageType::Cold, result_value);
            result.insert(DamageType::Electric, result_value);
            result.insert(DamageType::Water, result_value);
        },
        Stat::Endurance => {
            //piercing, crushing, cutting, poison end /10;
            let result_value = value / 10;
            result.insert(DamageType::Poison, result_value);
            result.insert(DamageType::Piercing, result_value);
            result.insert(DamageType::Crushing, result_value);
            result.insert(DamageType::Cutting, result_value);
        },
    }
    return result;
}

pub fn get_values_of_effect_resists_from_stat(stat: &Stat, value: i16) -> HashMap<EffectType, i16> {
    let mut result: HashMap<EffectType, i16> = HashMap::new();
    match *stat {
        Stat::Strength => {},
        Stat::Dexterity => {},
        Stat::Mobility => {},
        Stat::Wisdom => {},
        Stat::Intellect => {},
        Stat::Luck => {},
        Stat::Vitality => {
            //burn, electrification, freeze, wet, acid / 5;
            let result_value = value / 5;
            result.insert(EffectType::Acid, result_value);
            result.insert(EffectType::Burn, result_value);
            result.insert(EffectType::Electrification, result_value);
            result.insert(EffectType::Freeze, result_value);
            result.insert(EffectType::Wet, result_value);
        },
        Stat::Endurance => {
            //stun, moveless, slow, incresemovementspeed, bleeding, posion, frostbite /5;
            let result_value = value / 5;
            result.insert(EffectType::Bleeding, result_value);
            result.insert(EffectType::Stun, result_value);
            result.insert(EffectType::Moveless, result_value);
            result.insert(EffectType::IncreaseMovement, result_value);
            result.insert(EffectType::Poison, result_value);
            result.insert(EffectType::Frostbite, result_value);
            result.insert(EffectType::Slow, result_value);
        },
    }
    return result;
}
