use bevy::prelude::*;

use super::tile::Tile;
use super::Tilemap;
use crate::{components::{tile_component::{TileComponent, PermissionsComponent}, IdenteficationComponent}, resources::scene_manager::SceneManager};

pub fn cleanup(
    mut tile_query: Query<(Entity, &TileComponent, &PermissionsComponent, &IdenteficationComponent), With<TileComponent>>,
    mut commands: Commands,
    mut scene_manager: ResMut<SceneManager>,
) {
    let tilemap: &mut Tilemap = &mut scene_manager.get_current_game_scene_mut().tilemap;
    for (entity, tile_component, permissions_component, identification_component) in tile_query.iter_mut() {
        let tile: &mut Tile = tilemap.get_tile_by_index_mut(identification_component.id);
        copy_from_ground_component_to_tile(
            tile_component, 
            identification_component,
            permissions_component,
            tile
        ); 
        commands.entity(entity).despawn_recursive();
    }
}

pub fn copy_from_ground_component_to_tile(
    tile_component: &TileComponent, 
    identification_component: &IdenteficationComponent,
    permissions_component: &PermissionsComponent,
    tile: &mut Tile
) {
    tile.ground_type = tile_component.ground_type.clone();
    tile.ground_graphic_index = tile_component.ground_graphic_index;
    tile.cover_graphic_index = tile_component.cover_graphic_index;
    tile.cover_type = tile_component.cover_type.clone();

    tile.id = identification_component.id;

    tile.permissions = permissions_component.permissions.clone();
    tile.movement_ratio = permissions_component.momevement_ratio;
}
