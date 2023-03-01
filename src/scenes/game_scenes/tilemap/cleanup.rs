use bevy::prelude::*;

use crate::{components::tile_component::TileComponent, resources::scene_manager::SceneManager};
use super::Tilemap;
use super::tile::Tile;

pub fn cleanup(
    mut cover_query: Query<(Entity, &TileComponent), With<TileComponent>>,
    mut commands: Commands,
    mut scene_manager: ResMut<SceneManager>
){
    let tilemap: &mut Tilemap = &mut scene_manager.get_current_game_scene_mut().tilemap;
    for (cover_tile, tile_component) in cover_query.iter_mut() {
        let index = tile_component.index;
        let mut tile = tilemap.get_tile_by_index_mut(index);


        commands.entity(cover_tile).despawn_recursive();
    }
}
