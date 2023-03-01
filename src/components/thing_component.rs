use bevy::prelude::*;

#[derive(Component)]
pub struct ThingComponent{
    pub id: usize,
    pub tile_index: usize,
}