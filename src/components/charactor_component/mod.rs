use bevy::prelude::*;
use std::collections::HashMap;

//use crate::resources::scene_data::objects::charactor::skills::Skill;
use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::resources::scene_data::objects::body_part::{BodyPart, BodyPartType};
use crate::resources::scene_data::objects::resists::Resist;
use crate::resources::scene_data::objects::charactor::stats::Stat;
use crate::resources::scene_data::objects::stuff::Stuff;
//use crate::resources::scene_data::objects::charactor::charactor_effect::CharactorEffect;
use crate::resources::scene_data::objects::charactor::{StuffWearSlot, CharactorType, RaceType, AttitudeToPlayer};

#[derive(Component, Debug)]
pub struct CharactorComponent{
    pub id: usize,
    pub charactor_type: CharactorType,
    pub attitude_to_player: AttitudeToPlayer,
    //pub fraction: Fraction, // Maybe use this to create fights between NPCs; by default mosnters attacking NPCs and NPCs attacking monsters;
    pub race_type: RaceType,

    pub position: Position<i32>,
    pub graphic_position: Position<f32>,

    pub resists: HashMap<Resist, i16>,
    pub resists_cache: HashMap<Resist, i16>,
    pub resist_min_value: i16,
    pub resist_max_value: i16,

    pub stats: HashMap<Stat, u8>,
    pub stats_cache: HashMap<Stat, u8>,
    pub stat_min_value: u8,

    //pub skills: Vec<Skill>,
    //pub skills_cache: Vec<Skill>,

    pub stuff_storage: Vec<Stuff>,
    pub stuff_storage_max_slots: u8,
    pub stuff_wear: HashMap<StuffWearSlot, usize>, // stuff id;

    //pub charactor_effect: Vec<CharactorEffect>,

    pub body_structure: HashMap<BodyPartType, BodyPart>,
    pub current_health_points: i16, // cache from body_structure healthpoints
    pub total_health_points: i16,   // cache from body_structure healthpoints
}