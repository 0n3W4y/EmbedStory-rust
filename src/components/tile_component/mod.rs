use bevy::prelude::*;
//use bevy_inspector_egui::Inspectable;

use crate::scenes::game_scenes::tilemap::tile::{GroundType, CoverType};

#[derive(Component, Default)]
pub struct TileGroundComponent {
    pub ground_type: GroundType,
    pub index: usize, // in vec;
    pub ground_graphic_index: u8,
}

#[derive(Component, Default)]
pub struct TileCoverComponent{
    pub index: usize,
    pub cover_type: CoverType,
    pub cover_graphic_index: u8,
}
