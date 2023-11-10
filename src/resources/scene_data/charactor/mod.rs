use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::abilities::AbilityType;
use self::effects::Effect;
use self::skills::{Skill, SkillType};
use self::stats::Stat;
use crate::resources::scene_data::charactor::effects::EffectType;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::stuff::resists_types::ResistType;
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
pub mod damage_text_informer;

pub const STATS_EVERY_LEVEL: u8 = 2;
pub const STATS_MIN_VALUE: u8 = 1;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default, Hash)]
pub enum SkillSlot {
    #[default]
    Base,
    WeaponOne,
    WeaponTwo,
    WeaponThree,
    WeaponFour,
    BeltOne,
    BeltTwo,
    BeltThree,
    BeltFour,
    PotionHealth,
    PotionStamina,
}

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
    LeftHand,
    RightHand,
    Trinket,
    LeftRing,
    RightRing,
    Amulet,
    RightAndLeftHand,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
pub enum CharactorStatus {
    Dead,
    MovingLeft,
    MovingRight,
    MovingUp,
    MovingDown,
    #[default]
    Standing,
    CanAttack,
    TryAttack,
    AttackingLeft,
    AttackingRight,
    AttackingUp,
    AttackingDown,
    PickupingItem,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Charactor {
    pub id: usize,
    pub charactor_type: CharactorType,
    pub race_type: RaceType,
    pub gender_type: GenderType,

    pub status: CharactorStatus,

    //pub fraction: CharactorFraction,
    pub position: Position<i32>,
    pub destination_point: Option<Position<i32>>,
    pub destination_path: Vec<Position<i32>>,
    pub destination_direction: Position<i8>,

    pub level: u8,
    pub experience: u32,

    pub stats: HashMap<Stat, i16>,
    pub stats_cache: HashMap<Stat, i16>,

    pub resists: HashMap<ResistType, i16>,

    pub ability: HashMap<AbilityType, i16>,

    pub skills: HashMap<SkillSlot, Skill>,
    pub passive_skills: HashMap<SkillType, Skill>,

    pub stuff_storage: Vec<Stuff>,
    pub stuff_storage_max_slots: u8,
    pub stuff_wear: HashMap<StuffWearSlot, Option<Stuff>>,

    pub effects: HashMap<EffectType, Effect>,
}



//by default: if we have positive value -> we do damage; if we have negative value -> we add value (cure);
pub fn change_ability(
    ability_storage: &mut HashMap<AbilityType, i16>, 
    ability: &AbilityType,
    value: i16
) {
    ability_storage
        .entry(ability.clone())
        .and_modify(|old_value| *old_value -= value)
        .or_insert(value);
}

pub fn change_resist(
    resists: &mut HashMap<ResistType, i16>,
    resist_type: &ResistType,
    value: i16,
) {
    resists
        .entry(resist_type.clone())
        .and_modify(|old_value| *old_value -= value)
        .or_insert(value);
}

pub fn change_health_stamina_points_cache(
    stats: &mut HashMap<Stat, i16>,
    stats_cache: &mut HashMap<Stat, i16>,
    stat: &Stat,
    value: i16,
){
    let stat_value = match stats.get_mut(stat) {                    //chech for stat
        Some(v) => v,
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. Returned from the function", 
                stat,
            );
            return;                                                         // if we don't have this value -> returning;
        }
    };   
    let cache_value = match stats_cache.get_mut(stat) {                 //check for stat in cache;
        Some(v) => v,
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. Returned from function", 
                stat,
            );
            return;                                                              // if we don't have this value -> returning;
        }
    };
    
    *cache_value -= value;                                                  //set new value to cache;
    *stat_value -= value;                                                   // set new value to stat;

    if *cache_value <= 0 {                                                  //check for negative cache stat;
        println!("stat cache value <= 0!'{:?}'", stat);
    };
    if *stat_value < 1 {                                                 //check for minimal values for current values
       *stat_value = 1;                                                   // use this value, because we r changing CAHCE values;
    }
}

pub fn change_health_stamina_points(
    stats: &mut HashMap<Stat, i16>,
    stats_cache: &mut HashMap<Stat, i16>,
    stat: &Stat,
    value: i16,
){
    let cache_value = match stats_cache.get(stat) {                     //get cache value from stats;
        Some(v) => *v,
        _ => {
            println!(
                "Can not change stat: '{:?}', because stat is not in storage. Returning", 
                stat
            );
            return;         //if we don't have this value -> return from this and text message;
        }
    };
    let stat_value = match stats.get_mut(stat) {        //get current value from stat;
        Some(v) => v,
        _ => {
            println!(
                "Can not modify stat: '{:?}', because stat is not in storage. Returning", 
                stat,
            );
            return;         //if we don't have this calue -> return from this and text message;
        }
    };
    let temp_value = *stat_value - value;           //calculating stat value;

    if temp_value > cache_value && cache_value > 0{            //check value 
        *stat_value = cache_value;
    } else if cache_value < 1 && temp_value > 0{
        *stat_value = 1;
    } else {
        *stat_value = temp_value;
    }

    return;
}

pub fn change_stat(
    stats_storage: &mut HashMap<Stat, i16>,
    stats_cache: &mut HashMap<Stat, i16>,
    resists: &mut HashMap<ResistType, i16>,
    abilities: &mut HashMap<AbilityType, i16>,
    stat: &Stat,
    value: i16,
){
    let cache_value = match stats_cache.get_mut(stat) {             //get cache value;
        Some(v) => v,
        _ => {
            println!(
                "Can not change stat: '{:?}', because stat is not in cache storage. Returning",
                stat,
            );
            return;
        }
    };
    let stat_value = match stats_storage.get_mut(stat) {            //get stat;
        Some(v) => v,
        None => {
            println!(
                "Can not change stat: '{:?}', because stat is not in storage. Returning from the function",
                stat,
            );
            return;
        },
    };

    *cache_value -= value;                           //set new value to cache;
    let old_stat_value = *stat_value;           //storing old value to compare with new stat value;
    *stat_value -= value;                            //set new value to stat;

    if *cache_value < STATS_MIN_VALUE as i16{                   //check stat to minimal value;
        *stat_value = STATS_MIN_VALUE as i16;
    } else {
        *stat_value = *cache_value;
    }   
    
    if *stat_value != old_stat_value {                          //check for do dependences;
        do_stat_dependences(resists, abilities, stat,*stat_value,old_stat_value);
    }
}

pub fn do_stat_dependences(
    resists: &mut HashMap<ResistType, i16>,
    abilities: &mut HashMap<AbilityType, i16>,
    stat: &Stat,
    new_value: i16,
    old_value: i16,
) {
    let old_values_for_abilities: HashMap<AbilityType, i16> = abilities::get_values_of_abilities_from_stat(stat, old_value);
    let new_values_for_abilities: HashMap<AbilityType, i16> = abilities::get_values_of_abilities_from_stat(stat, new_value);
    for (ability_type, value) in new_values_for_abilities.iter() {
        let old_value_for_abilities =  old_values_for_abilities.get(ability_type).unwrap();         //safe call;
        let value_to_ability = old_value_for_abilities - value;                                 // if new value is bigger, negative value will add, positive will be substruct;
        change_ability(abilities, ability_type, value_to_ability);
    };

    let old_values_for_resist: HashMap<ResistType, i16> = get_values_of_resists_from_stat(stat, old_value);
    let new_values_for_resist: HashMap<ResistType, i16> = get_values_of_resists_from_stat(stat, new_value);
    for (resist_type, value) in new_values_for_resist.iter() {
        let old_value_for_resist = old_values_for_resist.get(resist_type).unwrap();                 //safe call;
        let value_to_resist = old_value_for_resist - value;                                     // if new value is bigger, negative value will add, positive will be substruct;
        change_resist(resists, resist_type, value_to_resist);
    };
}

//formulas
pub fn get_values_of_resists_from_stat(stat: &Stat, value: i16) -> HashMap<ResistType, i16> {
    let mut result: HashMap<ResistType, i16> = HashMap::new();
    match *stat {
        Stat::Strength => {
            let new_value = value / 10;         //formula for all resists: STR / 10;
            for resist_type in ResistType::all_values() {
                result.insert(resist_type.clone(), new_value);          //insert all values from ResistType enum;
            }
        },
        _ => {},
    }
    return result;
}

pub fn get_level_by_current_experience(experience: u32) -> u8 {         //formula to get new level;
    ((experience as f64).sqrt() / 6.0) as u8
}
