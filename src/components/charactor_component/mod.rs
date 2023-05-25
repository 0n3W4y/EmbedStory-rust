use bevy::prelude::*;
use std::collections::HashMap;

use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::resources::scene_data::objects::body_part::{BodyPart, BodyPartType};
use crate::resources::scene_data::objects::resists::Resist;
use crate::resources::scene_data::objects::charactor::stats::Stat;
use crate::resources::scene_data::objects::stuff::Stuff;
//use crate::resources::scene_data::objects::charactor::charactor_effect::CharactorEffect;
use crate::resources::scene_data::objects::charactor::{StuffWearSlot, CharactorType, RaceType, AttitudeToPlayer, CharactorFraction, CharactorStatus, CharactorSubType};
use crate::resources::scene_data::objects::charactor::skills::Skill;

#[derive(Component, Default)]
pub struct CharactorComponent{
    pub id: usize,
    pub charactor_type: CharactorType,
    pub charactor_subtype: CharactorSubType,
    pub attitude_to_player: AttitudeToPlayer,
    pub fraction: CharactorFraction,
    pub race_type: RaceType,

    pub status: CharactorStatus,

    pub position: Position<i32>,
    pub destination_point: Position<i32>,

    pub resists: HashMap<Resist, i16>,
    pub resists_cache: HashMap<Resist, i16>,
    pub resist_min_value: i16,
    pub resist_max_value: i16,

    pub stats: HashMap<Stat, u8>,
    pub stats_cache: HashMap<Stat, u8>,
    pub stat_min_value: u8,

    pub skills: HashMap<Skill, u16>,
    pub skills_cache: HashMap<Skill, u16>,

    pub stuff_storage: Vec<Stuff>,
    pub stuff_storage_max_slots: u8,
    pub stuff_wear: HashMap<StuffWearSlot, usize>, // stuff id;

    //pub charactor_effect: Vec<CharactorEffect>,

    pub body_structure: HashMap<BodyPartType, BodyPart>,
    pub current_health_points: i16, // cache from body_structure healthpoints
    pub total_health_points: i16,   // cache from body_structure healthpoints
}

#[derive(Component)]
pub struct PlayerComponent;

#[derive(Component)]
pub struct NPCComponent;

#[derive(Component)]
pub struct MonsterComponent;