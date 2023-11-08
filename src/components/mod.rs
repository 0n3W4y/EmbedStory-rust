use bevy::prelude::*;

use crate::scenes::game_scenes::tilemap::tile::Position;

pub mod tile_component;
pub mod thing_component;
pub mod charactor_component;
pub mod projectile_component;


#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum ObjectType{
    Charactor,
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
pub struct IdenteficationComponent {
    pub id: usize,
    pub object_type: ObjectType,
}