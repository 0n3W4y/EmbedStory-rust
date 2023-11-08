use bevy::prelude::*;

use crate::scenes::game_scenes::tilemap::tile::{GroundType, CoverType, TilePermissions};

#[derive(Component, Default)]
pub struct TileComponent {
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub cover_graphic_index: u8,
    pub ground_graphic_index: u8,
}

#[derive(Component, Default)]
pub struct PermissionsComponent {
    pub permissions: Vec<TilePermissions>,
    pub momevement_ratio: u16,
}