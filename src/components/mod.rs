use std::collections::HashMap;

use bevy::prelude::*;

use crate::{scenes::game_scenes::tilemap::tile::Position, resources::scene_data::{charactor::{CharactorType, skills::PassiveSkill, effects::Effect}, damage_text_informer::{DamageTextInformer, DamageIgnored}, Stat, Attribute, Resist, Damage}};

pub mod tile_component;
pub mod thing_component;
pub mod charactor_component;
pub mod projectile_component;
pub mod stuff_component;


#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum ObjectType{
    Charactor(CharactorType),
    Stuff,
    Thing,
    Projectile,
    #[default]
    Tile,
}

#[derive(Debug, Default)]
pub struct TakenDamage {
    pub damage: HashMap<Damage, i16>,
    pub effects: Vec<Effect>,
    pub passive_skills: Vec<PassiveSkill>,
    pub is_critical_hit: bool,
    pub missed_or_evaded: Option<DamageIgnored>,
}

#[derive(Component, Default)]
pub struct PositionComponent {
    pub position: Position<i32>
}

#[derive(Component, Default)]
pub struct IdentificationComponent {
    pub id: usize,
    pub object_type: ObjectType,
}

#[derive(Component, Default)]
pub struct TakenDamageComponent {
    pub damage: Vec<TakenDamage>,
}

#[derive(Component, Default)]
pub struct StatsComponent {
    pub stats: HashMap<Stat, i16>,
    pub stats_cache: HashMap<Stat, i16>,
}

#[derive(Component, Default)]
pub struct AttributesComponent {
    pub attributes: HashMap<Attribute, i16>,
    pub attributes_cache: HashMap<Attribute, i16>,
}

#[derive(Component, Default)]
pub struct ResistsComponent{
    pub resists: HashMap<Resist, i16>,
}

#[derive(Component, Default)]
pub struct DamageTextInformerComponent {
    pub text: Vec<DamageTextInformer>,
}