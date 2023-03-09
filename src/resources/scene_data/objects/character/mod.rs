use std::f32::consts::E;

use serde::{ Serialize, Deserialize };

use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::resources::scene_data::objects::character::stats::Stat;
use crate::resources::scene_data::objects::character::skills::Skill;

use super::body_part::BodyPart;
use super::resists::{Resist, MAX_RESIST_VALUE, MIN_RESIST_VALUE};

pub mod stats;
pub mod skills;

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default )]
pub enum CharacterType{
    Player,
    #[default]
    NPC,
    Monster,
    PlayerCompanion
}

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default )]
pub enum AttitudeToPlayer{
    #[default]
    Neutral,
    Enemy,
    Friendly
}

#[derive( PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default )]
pub enum RaceType{
    #[default]
    Human,
    Humanoid,
    Robot,
    Mutant,

}

#[derive( Serialize, Deserialize, Debug, Clone, Default )]
pub struct Character{
    pub id: usize,
    pub character_type: CharacterType,
    pub attitude_to_player: AttitudeToPlayer,
    pub race_type: RaceType,

    pub position: Position<i32>,
    pub graphic_position: Position<f32>,

    pub resists: Vec<Resist>,
    pub resists_cache: Vec<Resist>,

    pub stats: Vec<Stat>,
    pub stats_cache: Vec<Stat>,

    pub skills: Vec<Skill>,
    pub skill_cache: Vec<Skill>,

    
    pub body_structure: Vec<BodyPart>,
    pub current_health_points: i16, // cache
    pub total_health_points:i16, // cache
}

#[derive(Serialize, Deserialize)]
pub struct CharacterDeploy{

}

pub fn add_to_resist(character: &mut Character, resist: &Resist, value: i16){
    let resist = match character.resists
            .iter_mut()
            .find(|x|{*x == resist}) {
                Option::Some(v) => v,
                Option::None => {
                    println!(
                        "Character.add_to_resist; Can not add to '{:?}' value: {:?}, bacause resist is not in vec of resists.",
                        resist,
                        value
                    );
                    return;
                },
            };
    let new_value = resist.get_resist() + value;
    if new_value > MAX_RESIST_VALUE {
        resist.set_resist(MAX_RESIST_VALUE);
    } else {
        resist.set_resist(new_value);
    }
}

pub fn substruct_from_resist(character: &mut Character, resist: &Resist, value: i16){
    let resist = match character.resists
            .iter_mut()
            .find(|x|{*x == resist}) {
                Option::Some(v) => v,
                Option::None => {
                    println!(
                        "Character.add_to_resist; Can not add to '{:?}' value: {:?}, bacause resist is not in vec of resists.",
                        resist,
                        value
                    );
                    return;
                },
            };
    let new_value = resist.get_resist() - value;
    if new_value < MIN_RESIST_VALUE {
        resist.set_resist(MIN_RESIST_VALUE);
    } else {
        resist.set_resist(new_value);
    }
}
