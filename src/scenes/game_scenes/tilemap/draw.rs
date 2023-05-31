use bevy::prelude::*;

use crate::components::tile_component::{TileGroundComponent, TileCoverComponent};
use crate::materials::material_manager::MaterialManager;
use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::config::TILE_SIZE;

use super::tile::Tile;

pub const GROUND_Z_POSITION: f32 = 0.0;
pub const COVER_Z_POSITION: f32 = 1.0;

pub fn draw(
    mut commands: Commands,
    scene_manager: Res<SceneManager>,
    material_manager: Res<MaterialManager>,
) {
    let current_scene: &GameScene = scene_manager.get_current_game_scene();
    let tile_storage = current_scene.tilemap.get_tilemap_tile_storage();
    for tile in tile_storage.iter() {
        let x = tile.position.x as f32 * TILE_SIZE as f32;
        let y = tile.position.y as f32 * TILE_SIZE as f32;
        let ground_type = &tile.ground_type;
        let ground_transform = Transform::from_xyz(x, y, GROUND_Z_POSITION);
        let ground_texture: Handle<TextureAtlas> = material_manager
            .game_scene
            .tile
            .get_ground_atlas(ground_type);

        let mut ground_component: TileGroundComponent = Default::default();
        let mut cover_component: TileCoverComponent = Default::default();
        copy_from_tile_to_component( &mut ground_component, &mut cover_component, tile);

        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: ground_transform,
                sprite: TextureAtlasSprite::new(tile.ground_graphic_index as usize),
                texture_atlas: ground_texture,
                ..Default::default()
            })
            .insert(ground_component);

        let cover_type = &tile.cover_type;
        let cover_tranform: Transform = Transform::from_xyz(x, y, COVER_Z_POSITION);

      
        let cover_texture: Handle<TextureAtlas> = material_manager
        .game_scene
        .tile
        .get_cover_atlas(cover_type);
        
        commands
            .spawn_bundle(SpriteSheetBundle{
                transform: cover_tranform,
                sprite: TextureAtlasSprite::new(tile.cover_graphic_index as usize),
                texture_atlas: cover_texture,
                ..Default::default()
            })
            .insert(cover_component);
        
    }
}

pub fn copy_from_tile_to_component(ground_component: &mut TileGroundComponent, cover_component: &mut TileCoverComponent, tile: &Tile) {
    ground_component.ground_type = tile.ground_type.clone();
    ground_component.index = tile.index;
    ground_component.ground_graphic_index = tile.ground_graphic_index;

    cover_component.cover_graphic_index = tile.cover_graphic_index;
    cover_component.cover_type = tile.cover_type.clone();
    cover_component.index = tile.index;
}
