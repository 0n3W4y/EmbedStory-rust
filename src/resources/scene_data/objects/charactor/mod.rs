use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::resources::scene_data::objects::charactor::skills::Skill;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::body_part::{BodyPart, BodyPartType};
use super::resists::Resist;
use super::charactor::stats::Stat;
use super::stuff::Stuff;
use super::charactor::charactor_effect::CharactorEffect;

pub mod skills;
pub mod stats;
pub mod charactor_effect;
pub mod spawn;
pub mod draw;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum CharactorType {
    Player,
    #[default]
    NPC,
    Monster,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum GenderType{
    Female,
    #[default]
    Male,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum CharactorSubType{
    #[default]
    Civilian,
    MeleeFighter,
    RangedFighter,
    MixedFighter,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum AttitudeToPlayer {
    #[default]
    Neutral,
    Enemy,
    Friendly,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum RaceType {
    #[default]
    Human,
    Humanoid,
    Robot,
    Mutant,
    SuperMutant,
    Bogomol,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Hash)]
pub enum StuffWearSlot {
    Head,
    Vest,
    Pants,
    Gloves,
    Shoes,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Hash)]
pub enum ConditionType {
    Pain,
    Fatigue,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Charactor {
    pub id: usize,
    pub charactor_type: CharactorType,
    pub race_type: RaceType,
    pub charactor_subtype: CharactorSubType,    
    pub gender_type: GenderType,

    pub attitude_to_player: AttitudeToPlayer,
    //pub fraction: Fraction, // Maybe use this to create fights between NPCs; by default mosnters attacking NPCs and NPCs attacking monsters;    

    pub position: Position<i32>,
    pub graphic_position: Position<f32>,

    pub resists: HashMap<Resist, i16>,
    pub resists_cache: HashMap<Resist, i16>,
    pub resist_min_value: i16,
    pub resist_max_value: i16,

    pub stats: HashMap<Stat, u8>,
    pub stats_cache: HashMap<Stat, u8>,
    pub stat_min_value: u8,

    pub condition: HashMap<ConditionType, u16>,
    pub condition_max: HashMap<ConditionType, u16>,

    pub skills: HashMap<Skill, u16>,
    pub skills_cache: HashMap<Skill, u16>,

    pub stuff_storage: Vec<Stuff>,
    pub stuff_storage_max_slots: u8,
    pub stuff_wear: HashMap<StuffWearSlot, usize>, // value is - stuff id;

    pub charactor_effect: Vec<CharactorEffect>,

    pub body_structure: HashMap<BodyPartType, BodyPart>,
    pub current_health_points: i16, // cache from body_structure healthpoints
    pub total_health_points: i16,   // cache from body_structure healthpoints
}

pub fn change_resist(
    charactor_resist: &mut HashMap<Resist, i16>,
    charactor_resist_cache: &mut HashMap<Resist, i16>,
    target_resist: &Resist,
    value: i16,
    resist_min_value: i16,
    resist_max_value: i16,
) {
    //working with cache resist;
    //set cache value;
    match charactor_resist_cache.get_mut(target_resist){
        Option::Some(v) => *v += value,
        Option::None => {
            println!(
                    "Character.change_resist; Can't change: '{:?}'; with value: {:?}, bacause key not available in cache.",
                    target_resist,
                    value
                );
            return;
        }
    };

    //working with current;
    //get mut value of current;
    let resist = match charactor_resist.get_mut(target_resist) {
        Option::Some(v) => v,
        Option::None => {
            println!(
                "Character.change_resist; Can't get: '{:?}', bacause key not available.",
                target_resist,
            );
        return;
        }
    };

    // safe, because before we check it;
    let cache_resist = charactor_resist_cache.get(target_resist).unwrap();

    //calculate current value;
    if *cache_resist < resist_min_value {
        *resist = resist_min_value;
    } else if *cache_resist > resist_max_value {
        *resist = resist_max_value;
    } else {
        *resist = *cache_resist;
    }
}

pub fn change_stat(
    charactor_stat: &mut HashMap<Stat, u8>,
    charactor_stat_cache: &mut HashMap<Stat, i8>,
    target_stat: &Stat,
    value: i8,
    stat_min_value: u8,
) {
    //working with cache;
    //setting value to cache;
    match charactor_stat_cache.get_mut(target_stat) {
        Some(v) => *v += value,
        None => {
            println!(
                "Character.change_stat; Can't change '{:?}'; with value: {:?}, bacause key not available in cache.",
                target_stat,
                value
            );
            return;
        }
    }
    let stat = match charactor_stat.get_mut(target_stat) {
        Option::Some(v) => v,
        Option::None => {
            println!(
                    "Character.change_stat; Can't get '{:?}', bacause key not available.",
                    target_stat
                );
            return;
        }
    };

    //safe getter, check before;
    //get cache value;
    let stat_cache = charactor_stat_cache.get(target_stat).unwrap();

    //calculate current value;
    if *stat_cache < stat_min_value as i8 {
        *stat = stat_min_value;
    } else {
        *stat = *stat_cache as u8;
    }
}

pub fn calculate_current_health_points(body_structure: &HashMap<BodyPartType, BodyPart>) -> i16{
    let mut sum: i16 = 0;
    for (body_part_type, body_part) in body_structure {
        let hp = match body_part.get_current_health_points() {
            Ok(v) => v,
            Err(e) => {
                println!("Can't calculate current health points, because {} from {:?}, using default = 0", e, body_part_type);
                0
            }
        };

        sum += hp;
    }

    return sum;
}

pub fn calculate_total_health_points(body_structure: &HashMap<BodyPartType, BodyPart>) -> i16{
    let mut sum: i16 = 0;
    for (body_part_type, body_part) in body_structure {
        let hp = match body_part.get_total_health_points() {
            Ok(v) => v,
            Err(e) => {
                println!("Can't calculate total health points, because {} from {:?}, using default = 0", e, body_part_type);
                0
            }
        };

        sum += hp;
    }

    return sum;
}