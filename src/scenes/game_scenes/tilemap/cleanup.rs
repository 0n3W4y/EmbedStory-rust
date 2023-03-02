use bevy::prelude::*;

use super::tile::Tile;
use super::Tilemap;
use crate::{components::tile_component::TileComponent, resources::scene_manager::SceneManager};

pub fn cleanup(
    mut cover_query: Query<(Entity, &TileComponent), With<TileComponent>>,
    mut commands: Commands,
    mut scene_manager: ResMut<SceneManager>,
) {
    let tilemap: &mut Tilemap = &mut scene_manager.get_current_game_scene_mut().tilemap;
    for (cover_tile, tile_component) in cover_query.iter_mut() {
        let mut tile: &mut Tile = tilemap.get_tile_by_index_mut(tile_component.index);
        tile.ground_type = tile_component.ground_type.clone();
        tile.cover_type = tile_component.cover_type.clone();

        //no need to be cloned, because all tiles static;
        //tile.position = tile_component.position.clone();
        //tile.graphic_position = tile_component.graphic_position.clone();

        tile.cover_graphic_index = tile.cover_graphic_index;
        tile.movement_ratio = tile.movement_ratio;

        tile.permissions = tile.permissions.to_vec();

        tile.thing_type = tile.thing_type.clone(); // ( thing type, id of thing);
        tile.stuff_type = tile.stuff_type.clone();
        tile.character_type = tile.character_type.clone();
        tile.effect_type = tile.effect_type.clone();

        commands.entity(cover_tile).despawn_recursive();
    }
}
