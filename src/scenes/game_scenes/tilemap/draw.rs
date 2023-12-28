use bevy::prelude::*;

use crate::components::{PositionComponent, IdentificationComponent, ObjectType};
use crate::components::tile_component::{TileComponent, PermissionsComponent};
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
        let cover_type = &tile.cover_type;
        let transform = Transform::from_xyz(x, y, COVER_Z_POSITION);
        let texture: Handle<TextureAtlas> = material_manager.game_scene.tile.get_cover_atlas(cover_type);

        let mut tile_component: TileComponent = Default::default();
        let mut position_component: PositionComponent = Default::default();
        let mut identification_component: IdentificationComponent = Default::default();
        let mut permissions_component: PermissionsComponent = Default::default();
        copy_from_tile_to_component( &mut tile_component, &mut position_component, &mut identification_component, &mut permissions_component, tile);

        commands
            .spawn((SpriteSheetBundle {
                transform,
                sprite: TextureAtlasSprite::new(tile.cover_graphic_index as usize),
                texture_atlas: texture,
                ..Default::default()
            },
            tile_component,
            position_component,
            identification_component,
            permissions_component,
            ))
            .with_children(|parent| {
                let ground_texture: Handle<TextureAtlas> = material_manager.game_scene.tile.get_ground_atlas(ground_type);
                let ground_transform = Transform::from_xyz(0.0, 0.0, GROUND_Z_POSITION);
                parent.spawn(SpriteSheetBundle{
                    transform: ground_transform,
                    sprite: TextureAtlasSprite::new(tile.ground_graphic_index as usize),
                    texture_atlas: ground_texture,
                    ..Default::default()
                });
            });
    }
}

pub fn copy_from_tile_to_component(
    tile_component: &mut TileComponent, 
    position_component: &mut PositionComponent, 
    identification_component: &mut IdentificationComponent,
    permissions_component: &mut PermissionsComponent,
    tile: &Tile
) {
    tile_component.ground_type = tile.ground_type.clone();
    tile_component.ground_graphic_index = tile.ground_graphic_index;
    tile_component.cover_graphic_index = tile.cover_graphic_index;
    tile_component.cover_type = tile.cover_type.clone();

    position_component.position = tile.position.clone();

    identification_component.object_type = ObjectType::Tile(tile.id);

    permissions_component.permissions = tile.permissions.clone();
    permissions_component.momevement_ratio = tile.movement_ratio;
}
