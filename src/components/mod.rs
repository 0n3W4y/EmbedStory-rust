use std::collections::HashMap;

use bevy::prelude::*;

use crate::{scenes::game_scenes::tilemap::tile::Position, resources::scene_data::{charactor::{skills::PassiveSkill, effects::Effect, CharactorType}, damage_text_informer::{DamageTextInformer, DamageIgnored}, Stat, Attribute, Resist, Damage, Ability}};

pub mod tile_component;
pub mod thing_component;
pub mod charactor_component;
pub mod projectile_component;
pub mod stuff_component;


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ObjectType{
    Charactor(CharactorType, usize),
    Stuff(usize),
    Thing(usize),
    Projectile(usize),
    Tile(usize),
}

impl Default for ObjectType {
    fn default() -> Self {
        Self::Tile(0)
    }
}

#[derive(Debug, Default, Clone)]
pub struct TakenDamage {
    pub damage: HashMap<Damage, i16>,
    pub effects: Vec<Effect>,
    pub passive_skills: Vec<PassiveSkill>,
    pub is_critical_hit: bool,
    pub missed_or_evaded: Option<DamageIgnored>,
    pub area_of_impact: u8,
    pub no_evade: bool,
    pub no_block: bool,
}

#[derive(Component, Default)]
pub struct PositionComponent {
    pub position: Position<i32>,
    pub destination_point: Option<Position<i32>>,
    pub destination_path: Vec<Position<i32>>,
    pub destination_direction: Position<i8>,
}

#[derive(Component, Default)]
pub struct IdentificationComponent {
    pub object_type: ObjectType,
}

#[derive(Component, Default)]
pub struct TakenDamageComponent {
    pub damage: Vec<TakenDamage>,
    pub text: Vec<DamageTextInformer>,
}

#[derive(Component, Default)]
pub struct StatsComponent {
    pub stats: HashMap<Stat, i16>,
    pub stats_cache: HashMap<Stat, i16>,
    pub attributes: HashMap<Attribute, i16>,
    pub attributes_cache: HashMap<Attribute, i16>,
    pub resists: HashMap<Resist, i16>,
    pub ability: HashMap<Ability, i16>,
}