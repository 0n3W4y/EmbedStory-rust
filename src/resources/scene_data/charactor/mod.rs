use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::resists::{DamageResistType, EffectResistType};
use self::skills::{ActiveSkill, PassiveSkill};
use self::stats::{ExtraStat, Stat};
use crate::resources::scene_data::charactor::effects::EffectType;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::stuff::Stuff;

pub mod cleanup;
pub mod draw;
pub mod effects;
pub mod killed_charactor_handler;
pub mod move_charactor;
pub mod player_click_function;
pub mod resists;
pub mod skills;
pub mod stats;

pub const STATS_EVERY_LEVEL: u8 = 2;

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
    pub stats_min_value: u8,

    pub extra_stats: HashMap<ExtraStat, i16>,

    pub damage_resists: HashMap<DamageResistType, i16>,
    pub damage_resists_cache: HashMap<DamageResistType, i16>,
    pub damage_resists_min_value: i16,
    pub damage_resists_max_value: i16,

    pub effect_resists: HashMap<EffectResistType, i16>,
    pub effect_resists_cache: HashMap<EffectResistType, i16>,
    pub effect_resist_min_value: i16,
    pub effect_resist_max_value: i16,

    pub active_skills: HashMap<u8, Option<ActiveSkill>>,
    pub passive_skills: HashMap<PassiveSkill, i16>,

    pub stuff_storage: Vec<Stuff>,
    pub stuff_storage_max_slots: u8,
    pub stuff_wear: HashMap<StuffWearSlot, usize>, // value is - stuff id;

    pub short_time_effect: Vec<EffectType>,
    pub long_time_effect: Vec<EffectType>,
}

pub fn change_passive_skill(passive_skill_storage: &mut HashMap<PassiveSkill, i16>, passive_skill: &PassiveSkill, value: i16) {
    passive_skill_storage.entry(passive_skill.clone()).and_modify(|old_value| *old_value += value).or_insert(value);
}

pub fn change_effect_resist(
    effect_resists_storage: &mut HashMap<EffectResistType, i16>, 
    effect_resists_cache: &mut HashMap<EffectResistType, i16>,
    effect_resist: &EffectResistType,
    value: i16,
    effect_resist_max_value: i16,
    effect_resist_min_value: i16,
) {
    // if key is not in storage, we are added it to;
    effect_resists_cache.entry(effect_resist.clone()).and_modify(|old_value| *old_value += value).or_insert(value);
    let cache_value = effect_resists_cache.get(effect_resist).unwrap(); // safe

    let new_value = if *cache_value > effect_resist_max_value {
        effect_resist_max_value
    } else if *cache_value < effect_resist_min_value {
        effect_resist_min_value
    } else {
        *cache_value
    };

    effect_resists_storage.entry(effect_resist.clone()).and_modify(|old_value| *old_value = new_value).or_insert(new_value);
}

pub fn change_damage_resist(
    damage_resists_storage: &mut HashMap<DamageResistType, i16>,
    damage_resists_cache: &mut HashMap<DamageResistType, i16>,
    damage_resist: &DamageResistType,
    value: i16,
    damage_resist_max_value: i16,
    damage_resist_min_value: i16,
) {
    // if key is not in storage, we are added it to;
    damage_resists_cache.entry(damage_resist.clone()).and_modify(|old_value| *old_value += value).or_insert(value);
    let cache_value = damage_resists_cache.get(damage_resist).unwrap(); // safe;

    let new_value = if *cache_value > damage_resist_max_value {
        damage_resist_max_value
    } else if *cache_value < damage_resist_min_value {
        damage_resist_min_value
    } else {
        *cache_value
    };
}

pub fn change_extra_stat(
    extra_stats_storage: &mut HashMap<ExtraStat, i16>,
    extra_stat: &ExtraStat,
    value: i16,
) {
    extra_stats_storage.entry(extra_stat.clone()).and_modify(|old_value| *old_value += value).or_insert(value);
    match extra_stats_storage.get_mut(extra_stat) {
        Some(v) => {*v += value},
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. I created new entry with value:'{:?}'", 
                extra_stat,
                value
            );
            extra_stats_storage.insert(extra_stat.clone(), value);
        }
    };
}

pub fn change_stat(
    stats_storage: &mut HashMap<Stat, i16>,
    stats_cache: &mut HashMap<Stat, i16>,
    extra_stats_storage: &mut HashMap<ExtraStat, i16>,
    effect_resists_storage: &mut HashMap<EffectResistType, i16>, 
    effect_resists_cache: &mut HashMap<EffectResistType, i16>,
    effect_resists_min_value: i16,
    effect_resists_max_value: i16,
    damage_resists_storage: &mut HashMap<DamageResistType, i16>,
    damage_resists_cache: &mut HashMap<DamageResistType, i16>,
    damage_resist_max_value: i16,
    damage_resist_min_value: i16,
    passive_skill_storage: &mut HashMap<PassiveSkill, i16>,
    stat: &Stat,
    value: i16,
    stats_min_value: u8,
) {
    match stats_cache.get_mut(stat) {
        Some(v) => {*v += value},
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

    stats_storage.entry(stat.clone()).and_modify(|old_value| *old_value = new_value as i16).or_insert(new_value as i16);

    do_stat_dependences(
        extra_stats_storage,
        effect_resists_storage,
        effect_resists_cache,
        effect_resists_min_value,
        effect_resists_max_value,
        damage_resists_storage,
        damage_resists_cache,
        damage_resist_max_value,
        damage_resist_min_value,
        passive_skill_storage,
        stat,
        new_value,
    );
}

pub fn get_level_by_current_experience(experience: u32) -> u8 {
   let level: u8 = (experience as f64).sqrt() / 6.0;
}

pub fn do_stat_dependences(
    extra_stats_storage: &mut HashMap<ExtraStat, i16>,
    effect_resists_storage: &mut HashMap<EffectResistType, i16>, 
    effect_resists_cache: &mut HashMap<EffectResistType, i16>,
    effect_resists_min_value: i16,
    effect_resists_max_value: i16,
    damage_resists_storage: &mut HashMap<DamageResistType, i16>,
    damage_resists_cache: &mut HashMap<DamageResistType, i16>,
    damage_resist_max_value: i16,
    damage_resist_min_value: i16,
    passive_skill_storage: &mut HashMap<PassiveSkill, i16>,
    stat: &Stat, 
    stat_value: i16,
){
    match *stat {
        Stat::Dexterity => {},
        Stat::Endurance => {},
        Stat::Intellect => {},
        Stat::Luck => {},
        Stat::Mobility => {},
        Stat::Strength => {},
        Stat::Vitality => {},
        Stat::Wisdom => {},
    }
}

