use bevy::prelude::*;

use crate::scenes::game_scenes::tilemap::tile::CoverType;

#[derive(Component, Default)]
pub struct TileCoverComponent{
    pub cover_type: CoverType
}