use bevy::prelude::*;

use super::tile::Tile;
use super::Tilemap;
use crate::{components::tile_component::{TileGroundComponent, TileCoverComponent}, resources::scene_manager::SceneManager};

pub fn cleanup(
    mut ground_query: Query<(Entity, &TileGroundComponent), With<TileGroundComponent>>,
    mut cover_query: Query<(Entity, &TileCoverComponent), With<TileCoverComponent>>,
    mut commands: Commands,
    mut scene_manager: ResMut<SceneManager>,
) {
    let tilemap: &mut Tilemap = &mut scene_manager.get_current_game_scene_mut().tilemap;
    for (entity, ground_component) in ground_query.iter_mut() {
        let tile: &mut Tile = tilemap.get_tile_by_index_mut(ground_component.index);
        copy_from_ground_component_to_tile(tile, ground_component); 
        commands.entity(entity).despawn_recursive();
    }

    for (entity, cover_component) in cover_query.iter_mut() {
        let tile: &mut Tile = tilemap.get_tile_by_index_mut(cover_component.index);
        copy_from_cover_component_to_tile(tile, cover_component); 
        commands.entity(entity).despawn_recursive();
    }
}

pub fn copy_from_cover_component_to_tile(tile: &mut Tile, cover_component: &TileCoverComponent){
    tile.cover_type = cover_component.cover_type.clone();
    tile.cover_graphic_index = cover_component.cover_graphic_index;
}

pub fn copy_from_ground_component_to_tile( tile: &mut Tile, ground_component: &TileGroundComponent){
    tile.ground_type = ground_component.ground_type.clone();
    tile.ground_graphic_index = ground_component.ground_graphic_index;
}
