use serde::{Deserialize, Serialize};

use crate::resources::scene_data::objects::charactor::skills::Skill;
use crate::scenes::game_scenes::tilemap::tile::Position;

use super::body_part::BodyPart;
use super::resists::{Resist, MAX_RESIST_VALUE, MIN_RESIST_VALUE};
use super::charactor::stats::{Stat, MIN_STAT_VALUE};

pub mod skills;
pub mod stats;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub enum CharactorType {
    Player,
    #[default]
    NPC,
    Monster,
    PlayerCompanion,
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
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Charactor {
    pub id: usize,
    pub charactor_type: CharactorType,
    pub attitude_to_player: AttitudeToPlayer,
    pub race_type: RaceType,

    pub position: Position<i32>,
    pub graphic_position: Position<f32>,

    pub resists: Vec<Resist>,
    pub resists_cache: Vec<Resist>,

    pub stats: Vec<Stat>,
    pub stats_cache: Vec<Stat>,

    pub skills: Vec<Skill>,
    pub skills_cache: Vec<Skill>,

    pub body_structure: Vec<BodyPart>,
    pub current_health_points: i16, // cache
    pub total_health_points: i16,   // cache
}

#[derive(Serialize, Deserialize)]
pub struct CharacterDeploy {}

pub fn add_to_resist(
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
                    "Character.add_to_resist; Can not add to '{:?}' value: {:?}, bacause resist is not in vec of resists.",
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
                    "Character.add_to_resist; Can not add to '{:?}' value: {:?}, bacause resist is not in vec of resists_cache.",
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

pub fn substruct_from_resist(
    charactor_resist: &mut Vec<Resist>,
    charactor_resist_cache: &mut Vec<Resist>,
    target_resist: &Resist,
    value: i16,
) {
    //working with cashe;
    let resist = match charactor_resist.iter_mut().find(|x| *x == target_resist) {
        Option::Some(v) => v,
        Option::None => {
            println!(
                    "Character.substruct_from_resist; Can not add to '{:?}' value: {:?}, bacause resist is not in vec of resists.",
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
                    "Character.substruct_from_resist; Can not add to '{:?}' value: {:?}, bacause resist is not in vec of resists_cache.",
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
    } else {
        resist.set_resist(cache_value);
    }
}

pub fn add_to_stat(
    charactor_stat: &mut Vec<Stat>,
    charactor_stat_cache: &mut Vec<Stat>,
    target_stat: &Stat,
    value: i16
) {
    let stat = match charactor_stat.iter_mut().find(|x| *x == target_stat) {
        Option::Some(v) => v,
        Option::None => {
            println!(
                    "Character.add_to_stat; Can not add to '{:?}' value: {:?}, bacause stat is not in vec of stats.",
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
                    "Character.substruct_from_resist; Can not add to '{:?}' value: {:?}, bacause resist is not in vec of resists_cache.",
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

pub fn substruct_from_stat(
    charactor_stat: &mut Vec<Stat>,
    charactor_stat_cache: &mut Vec<Stat>,
    target_stat: &Stat,
    value: i16,
) {
    //working with cache;
    let stat = match charactor_stat.iter_mut().find(|x| *x == target_stat) {
        Option::Some(v) => v,
        Option::None => {
            println!(
                    "Character.add_to_stat; Can not add to '{:?}' value: {:?}, bacause stat is not in vec of stats.",
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
                    "Character.substruct_from_resist; Can not add to '{:?}' value: {:?}, bacause resist is not in vec of resists_cache.",
                    target_stat,
                    value
                );
            return;
        }
    };
    //set cache value;
    let cache_value = cache_stat.get_stat() - value;
    cache_stat.set_stat(cache_value);

    //calculate current value;
    if cache_value < MIN_STAT_VALUE {
        stat.set_stat(MIN_STAT_VALUE)
    } else {
        stat.set_stat(cache_value);
    }
}