use bevy::prelude::*;
use std::collections::HashMap;

use crate::resources::scene_data::AbilityType;
use crate::resources::scene_data::charactor::CharactorStrength;
use crate::resources::scene_data::charactor::SkillSlot;
use crate::resources::scene_data::charactor::skills::SkillType;
use crate::scenes::game_scenes::tilemap::tile::Position;
use crate::resources::scene_data::charactor::effects::Effect;
use crate::resources::scene_data::charactor::effects::EffectType;
use crate::resources::scene_data::charactor::skills::Skill;
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
    Use,
    #[default]
    None
}

#[derive(Component, Default)]
pub struct CharactorAnimationComponent {
    //pub animation_type: CharactorAnimationType,
    pub duration: f32,
}

#[derive(Component, Default)]
pub struct EffectComponent{
    pub effects: HashMap<EffectType, Effect>,
}

#[derive(Component, Default)]
pub struct DestinationComponent {
    pub destination_point: Option<Position<i32>>,
    pub destination_path: Vec<Position<i32>>,
    pub destination_direction: Position<i8>,
}

#[derive(Component, Default)]
pub struct AbilityComponent {
    pub ability: HashMap<AbilityType, i16>,
}

#[derive(Component, Default)]
pub struct SkillComponent {
    pub skills: HashMap<SkillSlot, Skill>,
    pub passive_skills: HashMap<SkillType, Skill>,
}

#[derive(Component, Default)]
pub struct InventoryComponent {
    pub stuff_storage: Vec<Stuff>,
    pub stuff_storage_max_slots: u8,
    pub stuff_wear: HashMap<StuffWearSlot, Option<Stuff>>, // value is - stuff id;
}

#[derive(Component, Default)]
pub struct CharactorComponent{
    pub charactor_type: CharactorType,
    pub race_type: RaceType,
    pub gender_type: GenderType,
    pub strength: CharactorStrength,

    pub status: CharactorStatus,
    //pub fraction: CharactorFraction,
    pub level: u8,
    pub experience: u32,
}

#[derive(Component, Default)]
pub struct CharactorTargetComponent {
    pub target: Option<usize>,
    pub target_position: Option<Position<i32>>,
    pub action: ActionType,
}

#[derive(Component, Default)]
pub struct PlayerComponent;

#[derive(Component, Default)]
pub struct NPCComponent;

#[derive(Component, Default)]
pub struct MonsterComponent;

#[derive(Component, Default)]
pub struct CompanionComponent;