use std::collections::HashMap;

use bevy::prelude::*;

use crate::{scenes::game_scenes::tilemap::tile::Position, resources::scene_data::{charactor::CharactorType, damage_text_informer::DamageTextInformer, Stat, Attribute, stuff::resists_types::ResistType}};

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
pub struct DamageTextComponent {
    pub text_upper: Vec<DamageTextInformer>,
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
    pub resists: HashMap<ResistType, i16>,
}