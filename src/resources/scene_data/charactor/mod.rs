use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::effects::{EffectType, Effect, EffectStatus};
use self::skills::{ActiveSkill, PassiveSkillType, PassiveSkill};
use crate::components::{StatsComponent, AttributesComponent};
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::{Stat, Attribute, Ability, Resist, Damage};
use super::stuff::Stuff;

pub mod cleanup;
pub mod draw;
pub mod effects;
pub mod killed_charactor_handler;
pub mod update_move;
pub mod player_click_function;
pub mod skills;
pub mod update_effects;
pub mod update_passive_skills;
pub mod update_attack;
pub mod update_cooldowns;
pub mod active_skill_handler;

pub const STATS_POINTS_EVERY_LEVEL: u8 = 2;
pub const STATS_MIN_VALUE: u8 = 1;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default, Hash)]
pub enum SkillSlot {
    #[default]
    Base,
    WeaponOne,
    WeaponTwo,
    WeaponThree,
    WeaponFour,
    TrinketOne,
    TrinketTwo,
    BeltOne,
    BeltTwo,
    BeltThree,
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
pub enum RaceType {
    #[default]
    Human,
    Elf,
    Orc,
    Dwarf,
    Halfling,
    Lizardfolk,
    Naga,
    Gnome,
    Goblin,
    Minotaur,
    Harpia,
    Dryada,
    Fairy,
    Celestial,
    Elemental,
    Ghost,
    Skeleton,
    Zombie,
    Ogre,
    Demon,
    Wolf,
    Bear,
    Crocodile,
    Scorpion,
    Eagle,
    Spider,
    KomodoDragon,
    Rhinocerops,
    Snake,
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
    AttackingLeft,
    AttackingRight,
    AttackingUp,
    AttackingDown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
pub enum CharactorStrength {
    Weak,
    #[default]
    Normal,
    Champion,
    Elite,
    Boss,
    None,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Charactor {
    pub id: usize,
    pub charactor_type: CharactorType,
    pub race_type: RaceType,
    pub gender_type: GenderType,
    pub strength: CharactorStrength,

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

    pub attributes: HashMap<Attribute, i16>,
    pub attributes_cache: HashMap<Attribute, i16>,

    pub resists: HashMap<Resist, i16>,

    pub ability: HashMap<Ability, i16>,

    pub base_skill: ActiveSkill,
    pub active_skills: HashMap<SkillSlot, ActiveSkill>,
    pub passive_skills: HashMap<PassiveSkillType, PassiveSkill>,

    pub stuff_storage: Vec<Stuff>,
    pub stuff_storage_max_slots: u8,
    pub stuff_wear: HashMap<StuffWearSlot, Option<Stuff>>,

    pub effects: HashMap<EffectType, Effect>,
    pub effects_immunes: Vec<EffectType>,
    pub effect_status: Vec<EffectStatus>,
}



//by default: if we have positive value -> we do damage; if we have negative value -> we add value (cure);
pub fn change_ability(
    ability_storage: &mut HashMap<Ability, i16>, 
    ability: &Ability,
    value: i16
) {
    ability_storage
        .entry(ability.clone())
        .and_modify(|old_value| *old_value += value)
        .or_insert(value);
}

pub fn change_resist(
    resists: &mut HashMap<Resist, i16>,
    resist_type: &Resist,
    value: i16,
) {
    resists
        .entry(resist_type.clone())
        .and_modify(|old_value| *old_value += value)
        .or_insert(value);
}

pub fn change_attribute_points(
    attributes: &mut AttributesComponent,
    attribute: &Attribute,
    value: i16,
    change_cache: bool,
){
    if change_cache {
        let cache_value = match attributes.attributes_cache.get_mut(&attribute) {
            Some(v) => {
                *v += value;
                *v
            },
            None => {
                println!("Cannot change attribute, because attribute not in cache storage! {:?}", attribute);
                0
            },
        };

        match attributes.attributes.get_mut(&attribute) {
            Some(v) => {
                let new_value = *v - value;
                if new_value < 1 {
                    *v = 1;
                } else if new_value > cache_value {
                    *v = cache_value;
                } else {

                }
            },
            None => {
                println!("Cannot change attribute, because attribute not find in attribute storage! {:?}", attribute);
            }
        }; 
    } else {
        match attributes.attributes.get_mut(&attribute) {
            Some(v) => {
                *v += value;
            },
            None => {
                println!("Cannot change attribute, because attribute not find in attribute storage! {:?}", attribute);
            }
        }
    }   
}

pub fn change_stat_points(
    stats: &mut StatsComponent,
    resists: &mut HashMap<Resist, i16>,
    abilities: &mut HashMap<Ability, i16>,
    attributes: &mut AttributesComponent,
    stat: &Stat,
    value: i16,
){
    let cache_value = match stats.stats_cache.get_mut(stat) {             //get cache value;
        Some(v) => v,
        _ => {
            println!(
                "Can not change stat: '{:?}', because stat is not in cache storage. Returning",
                stat,
            );
            return;
        }
    };
    let stat_value = match stats.stats.get_mut(stat) {            //get stat;
        Some(v) => v,
        None => {
            println!(
                "Can not change stat: '{:?}', because stat is not in storage. Returning from the function",
                stat,
            );
            return;
        },
    };

    *cache_value += value;                           //set new value to cache;
    let old_stat_value = *stat_value;           //storing old value to compare with new stat value;
    *stat_value += value;                            //set new value to stat;

    if *cache_value < STATS_MIN_VALUE as i16{                   //check stat to minimal value;
        *stat_value = STATS_MIN_VALUE as i16;
    } else {
        *stat_value = *cache_value;
    }   
    
    if *stat_value != old_stat_value {                          //check for do dependences;
        do_stat_dependences(resists, abilities, attributes, stat,*stat_value,old_stat_value);
    }
}

pub fn do_stat_dependences(
    resists: &mut HashMap<Resist, i16>,
    abilities: &mut HashMap<Ability, i16>,
    attributes: &mut AttributesComponent,
    stat: &Stat,
    new_value: i16,
    old_value: i16,
) {
    if resists.len() != 0 {
        let old_values_for_abilities: HashMap<Ability, i16> = get_values_of_abilities_from_stat(stat, old_value);
        let new_values_for_abilities: HashMap<Ability, i16> = get_values_of_abilities_from_stat(stat, new_value);
        for (ability_type, value) in new_values_for_abilities.iter() {
            let old_value_for_abilities =  old_values_for_abilities.get(ability_type).unwrap();         //safe call;
            let value_to_ability = value - old_value_for_abilities;                                 // if new value is lower, negative value will add, positive will be substruct;
            change_ability(abilities, ability_type, value_to_ability);
        };
    }    

    if abilities.len() != 0 {
        let old_values_for_resist: HashMap<Resist, i16> = get_values_of_resists_from_stat(stat, old_value);
        let new_values_for_resist: HashMap<Resist, i16> = get_values_of_resists_from_stat(stat, new_value);
        for (resist_type, value) in new_values_for_resist.iter() {
            let old_value_for_resist = old_values_for_resist.get(resist_type).unwrap();                 //safe call;
            let value_to_resist = value - old_value_for_resist;                                     // if new value is bigger, negative value will add, positive will be substruct;
            change_resist(resists, resist_type, value_to_resist);
        };
    }

    
    let old_values_for_attributes = get_values_of_attributes_from_stat(stat, old_value);
    let new_values_for_attributes = get_values_of_attributes_from_stat(stat, new_value);
    for(attribute, value) in new_values_for_attributes.iter() {
        let old_value_for_attribute = old_values_for_attributes.get(attribute).unwrap();
        let value_to_attribute = value - old_value_for_attribute;
        change_attribute_points(attributes, attribute, value_to_attribute, true);
    }
}

//formulas
pub fn get_values_of_resists_from_stat(stat: &Stat, value: i16) -> HashMap<Resist, i16> {
    let mut result: HashMap<Resist, i16> = HashMap::new();
    match *stat {
        Stat::Strength => {
            let new_value = value / 10;         //formula for all resists: STR / 10;
            for resist_type in Resist::all_values() {
                result.insert(resist_type.clone(), new_value);          //insert all values from ResistType enum;
            }
        },
        _ => {},
    }
    return result;
}


pub fn get_values_of_attributes_from_stat(stat: &Stat, value: i16) -> HashMap<Attribute, i16> {
    //STR/2; DEX/4; INT/6; VIT*4
    //INT/2; DEX/4; STR/6; VIT*4
    let mut result: HashMap<Attribute, i16> = HashMap::new();
    match *stat {
        Stat::Strength => {
            result.insert(Attribute::Health, value / 2);
            result.insert(Attribute::Stamina, value / 6);
        },
        Stat::Dexterity => {
            result.insert(Attribute::Health, value / 4);
            result.insert(Attribute::Stamina, value / 4);
        },
        Stat::Wisdom => {
            result.insert(Attribute::Health, value / 6);
            result.insert(Attribute::Stamina, value / 2);
        },
        Stat::Luck => {},
        Stat::Vitality => {
            result.insert(Attribute::Health, value * 4);
            result.insert(Attribute::Stamina, value * 4);
        },
    }

    return result;
}
pub fn get_level_by_current_experience(experience: u32) -> u8 {         //formula to get new level;
    ((experience as f64).sqrt() / 6.0) as u8
}

pub fn get_values_of_abilities_from_stat(stat: &Stat, value: i16) -> HashMap<Ability, i16> {
    let mut result: HashMap<Ability, i16> = HashMap::new();
    match *stat {
        Stat::Dexterity => {
            let evasion = value / 15;             //evasion:  dex/15
            let movement_speed = value / 10;      //move speed: dex/10;
            let attack_speed = value / 10;        //atk speed: dex/10;
            result.insert(Ability::Evasion, evasion);
            result.insert(Ability::MovementSpeed, movement_speed);
            result.insert(Ability::AttackSpeed, attack_speed);
        },
        Stat::Wisdom => {
            let cooldown_active_skill = value / 10;           //cd of active skills: INT /10;
            let critical_multiplier = value / 2;              // Crit Multi : INT / 5;
            result.insert(Ability::ActiveSkillsCoolDawn, cooldown_active_skill);
            result.insert(Ability::CriticalHitMultiplier, critical_multiplier);

        },
        Stat::Luck => {
            let critical_hit_chance = value / 10;               //crit chance: LCK / 10;
            let block_chance = value / 10;                      //block chance: LCK /10;
            let accuracy = value / 10;                      //accuracy: LCK / 10;
            result.insert(Ability::CriticalHitChanse, critical_hit_chance);
            result.insert(Ability::BlockChance, block_chance);
            result.insert(Ability::Accuracy, accuracy);
        },
        Stat::Strength => {
            let block_amount = value / 10;              //block amount: STR / 10;
            result.insert(Ability::BlockAmount, block_amount);
        }
        Stat::Vitality => todo!(),
    }
    return result;
}

pub fn get_ability_type_from_damage_type (damage_type: &Damage) -> Ability {
    return match *damage_type {
        Damage::Fire => Ability::FireDamage,
        Damage::Cold => Ability::ColdDamage,
        Damage::Electric => Ability::ElectricDamage,
        Damage::Phisical => Ability::PhisicalDamage,
        Damage::Acid => Ability::AcidDamage,
        Damage::Poison => Ability::PoisonDamage,
        Damage::Water => Ability::WaterDamage,     
    }
}
