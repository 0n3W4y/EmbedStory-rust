use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::resources::scene_data::objects::charactor::skills::Skill;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::body_part::BodyPart;
use super::resists::Resist;
use super::charactor::stats::{Stat, MIN_STAT_VALUE};
use super::stuff::Stuff;
use super::charactor::charactor_effect::CharactorEffect;

pub mod skills;
pub mod stats;
pub mod charactor_effect;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy)]
pub enum CharactorType {
    Player,
    NPC(NPCType),
    Monster(MonsterType),
    PlayerCompanion(CompanionType),
}

impl Default for CharactorType {
    fn default() -> Self {
        Self::NPC(NPCType::Civilian)
    }
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum NPCType {
    #[default]
    Civilian,
    MeleeFighter,
    RangedFighter,
    MixedFighter,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum MonsterType{
    #[default]
    Bogomol,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum CompanionType{
    #[default]
    Scientist,
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
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Hash)]
pub enum StuffWearSlot {
    Head,
    Vest,
    Pants,
    Gloves,
    Shoes,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Charactor {
    pub id: usize,
    pub charactor_type: CharactorType,
    pub attitude_to_player: AttitudeToPlayer,
    //pub fraction: Fraction, // Maybe use this to create fights between NPCs; by default mosnters attacking NPCs and NPCs attacking monsters;
    pub race_type: RaceType,

    pub position: Position<i32>,
    pub graphic_position: Position<f32>,

    pub resists: HashMap<Resist, i16>,
    pub resists_cache: HashMap<Resist, i16>,
    pub min_resist_value: i16,
    pub max_resist_value: i16,

    pub stats: HashMap<Stat, u8>,
    pub stats_cache: HashMap<Stat, u8>,
    pub min_stat_value: u8,

    pub skills: Vec<Skill>,
    pub skills_cache: Vec<Skill>,

    pub stuff_storage: Vec<Stuff>,
    pub stuff_wear: Vec<Stuff>, // 
    pub stuff_wear_slots: Vec<StuffWearSlot>,

    pub charactor_effect: Vec<CharactorEffect>,

    pub body_structure: Vec<BodyPart>,
    pub current_health_points: i16, // cache from body_structure healthpoints
    pub total_health_points: i16,   // cache from body_structure healthpoints
}

#[derive(Serialize, Deserialize)]
pub struct RaceDeploy {}

#[derive(Serialize, Deserialize)]
pub struct NPCDeploy {}

pub fn change_resist(
    charactor_resist: &mut Vec<Resist>,
    charactor_resist_cache: &mut Vec<Resist>,
    target_resist: &Resist,
    value: i16,
) {
    //working with cache;
    let resist = match charactor_resist.iter_mut().find(|x| *x == target_resist) {
        Option::Some(v) => v,
        Option::None => {
            println!(
                    "Character.change_resist; Can no cnahge: '{:?}'; with value: {:?}, bacause resist is not in vec of resists.",
                    target_resist,
                    value
                );
            return;
        }
    };
    let cache_resist = match charactor_resist_cache
        .iter_mut()
        .find(|x| *x == target_resist)
    {
        Option::Some(v) => v,
        Option::None => {
            println!(
                    "Character.change_resist; Can no cnahge: '{:?}'; with value: {:?}, bacause resist is not in vec of resists_cache.",
                    target_resist,
                    value
                );
            return;
        }
    };
    //set cache value;
    let cache_value = cache_resist.get_resist() + value;
    cache_resist.set_resist(cache_value);

    //calculate current value;
    if cache_value < MIN_RESIST_VALUE {
        resist.set_resist(MIN_RESIST_VALUE);
    } else if cache_value > MAX_RESIST_VALUE {
        resist.set_resist(MAX_RESIST_VALUE);
    } else {
        resist.set_resist(cache_value);
    }
}

pub fn change_stat(
    charactor_stat: &mut Vec<Stat>,
    charactor_stat_cache: &mut Vec<Stat>,
    target_stat: &Stat,
    value: i16
) {
    let stat = match charactor_stat.iter_mut().find(|x| *x == target_stat) {
        Option::Some(v) => v,
        Option::None => {
            println!(
                    "Character.change_stat; Can not change '{:?}'; with value: {:?}, bacause stat is not in vec of stats.",
                    target_stat,
                    value
                );
            return;
        }
    };
    let cache_stat = match charactor_stat_cache
        .iter_mut()
        .find(|x| *x == target_stat)
    {
        Option::Some(v) => v,
        Option::None => {
            println!(
                    "Character.change_stat; Can not change '{:?}'; with value: {:?}, bacause stat is not in vec of stat_cache.",
                    target_stat,
                    value
                );
            return;
        }
    };
    //set cache value;
    let cache_value = cache_stat.get_stat() + value;
    cache_stat.set_stat(cache_value);

    //calculate current value;
    if cache_value < MIN_STAT_VALUE {
        stat.set_stat(MIN_STAT_VALUE);
    } else {
        stat.set_stat(cache_value);
    }
}