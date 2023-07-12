use bevy::prelude::*;
use std::collections::HashMap;

use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::resources::scene_data::charactor::effects::Effect;
use crate::resources::scene_data::charactor::stats::Stat;
use crate::resources::scene_data::charactor::stats::ExtraStat;
use crate::resources::scene_data::stuff::damage_type::DamageType;
use crate::resources::scene_data::charactor::effects::EffectType;
use crate::resources::scene_data::charactor::abilities::Ability;
use crate::resources::scene_data::charactor::skills::{Skill, SkillType};
use crate::resources::scene_data::stuff::Stuff;
use crate::resources::scene_data::charactor::StuffWearSlot;
use crate::resources::scene_data::charactor::{CharactorType, RaceType, GenderType, CharactorStatus};

#[derive(Default, Eq, PartialEq, Debug)]
pub enum ActionType {
    Attack,
    Pickup,
    Open,
    Talk,
    Move,
    #[default]
    None
}


#[derive(Component, Default)]
pub struct EffectComponent{
    pub temporary_effect: HashMap<EffectType, Effect>,
    pub endless_effect: HashMap<EffectType, Effect>,
}

#[derive(Component, Default)]
pub struct PositionComponent {
    pub position: Position<i32>,
    pub destination_point: Position<i32>,
    pub destination_path: Vec<Position<i32>>,
    pub destination_direction: Position<i8>,
}

#[derive(Component, Default)]
pub struct StatsComponent {
    pub stats: HashMap<Stat, i16>,
    pub stats_cache: HashMap<Stat, i16>,
    pub stats_min_value: u8,
}

#[derive(Component, Default)]
pub struct ExtraStatsComponent {
    pub extra_stats: HashMap<ExtraStat, i16>,
    pub extra_stats_cache: HashMap<ExtraStat, i16>,
    pub extra_stats_regen: HashMap<ExtraStat, f32>,
}


#[derive(Component, Default)]
pub struct ResistsComponent {
    pub damage_resists: HashMap<DamageType, i16>,
    pub damage_resists_cache: HashMap<DamageType, i16>,
    pub damage_resists_min_value: i16,
    pub damage_resists_max_value: i16,

    pub effect_resists: HashMap<EffectType, i16>,
    pub effect_resists_cache: HashMap<EffectType, i16>,
    pub effect_resists_min_value: i16,
    pub effect_resists_max_value: i16,
}

#[derive(Component, Default)]
pub struct AbilityComponent {
    pub ability: HashMap<Ability, f32>,
}

#[derive(Component, Default)]
pub struct SkillComponent {
    pub skills: HashMap<u8, Skill>,
    pub passive_skills: HashMap<SkillType, Skill>,
}

#[derive(Component, Default)]
pub struct InventoryComponent {
    pub stuff_storage: Vec<Stuff>,
    pub stuff_storage_max_slots: u8,
    pub stuff_wear: HashMap<StuffWearSlot, usize>, // value is - stuff id;
}

#[derive(Component, Default)]
pub struct CharactorComponent{
    pub id: usize,
    pub charactor_type: CharactorType,
    pub race_type: RaceType,
    pub gender_type: GenderType,

    pub status: CharactorStatus,
    pub target: Option<usize>,
    pub action: ActionType,

    //pub fraction: CharactorFraction,
    pub level: u8,
    pub experience: u32,
}

#[derive(Component, Default)]
pub struct PlayerComponent;

#[derive(Component, Default)]
pub struct NPCComponent;

#[derive(Component, Default)]
pub struct MonsterComponent;

#[derive(Component, Default)]
pub struct CompanionComponent;