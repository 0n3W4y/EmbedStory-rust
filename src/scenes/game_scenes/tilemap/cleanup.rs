use bevy::prelude::*;

use super::tile::Tile;
use super::Tilemap;
use crate::{components::tile_component::TileComponent, resources::scene_manager::SceneManager};
use crate::components::tile_component::tile_cover_component::TileCoverComponent;

pub fn cleanup(
    mut tile_query: Query<(Entity, &TileComponent, &TileCoverComponent), With<TileComponent>>,
    mut commands: Commands,
    mut scene_manager: ResMut<SceneManager>,
) {
    let tilemap: &mut Tilemap = &mut scene_manager.get_current_game_scene_mut().tilemap;
    for (entity, component, cover_component) in tile_query.iter_mut() {
        let tile: &mut Tile = tilemap.get_tile_by_index_mut(component.index);
        copy_from_entity_component_to_tile(tile, component, cover_component);        
        commands.entity(entity).despawn_recursive();
    }
}

pub fn copy_from_entity_component_to_tile(tile: &mut Tile, component: &TileComponent, cover_component: &TileCoverComponent){
    tile.cover_type = cover_component.cover_type.clone();

    tile.cover_graphic_index = component.cover_graphic_index;
    tile.movement_ratio = component.movement_ratio;
    tile.permissions = component.permissions.to_vec();
    tile.thing_type = component.thing_type.clone();
    tile.stuff_type = component.stuff_type.to_vec();
    tile.charactor_type = component.alive_charactor_type.clone();
    tile.effect_type = component.effect_type.clone();

}
