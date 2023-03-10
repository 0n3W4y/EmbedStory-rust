use bevy::prelude::*;

use crate::components::tile_component::TileComponent;
use crate::components::tile_component::tile_cover_component::TileCoverComponent;
use crate::materials::material_manager::MaterialManager;
use crate::resources::scene_manager::SceneManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::CoverType;

use super::tile::Tile;

pub const Z_POSITION: f32 = 0.0;

pub fn draw(
    mut commands: Commands,
    scene_manager: Res<SceneManager>,
    material_manager: Res<MaterialManager>,
) {
    let current_scene: &GameScene = scene_manager.get_current_game_scene();
    let tile_storage = current_scene.tilemap.get_tilemap_tile_storage();
    for tile in tile_storage.iter() {
        let x = tile.graphic_position.x;
        let y = tile.graphic_position.y;
        let ground_type = &tile.ground_type;
        let ground_transform = Transform::from_xyz(x as f32, y as f32, Z_POSITION);
        let ground_texture: Handle<Image> = material_manager
            .game_scene
            .ground_tile
            .get_image(ground_type)
            .clone();

        let component: TileComponent = Default::default();
        let cover_component: TileCoverComponent = Default::default();

        commands
            .spawn_bundle(SpriteBundle {
                transform: ground_transform,
                texture: ground_texture,
                ..Default::default()
            })
            .with_children(|parent| {
                let cover_type = &tile.cover_type;
                let cover_tranform: Transform = Transform::from_xyz(0.0, 0.0, Z_POSITION + 1.0);
                if *cover_type != CoverType::None {
                    let cover_texture: Handle<Image> = material_manager
                        .game_scene
                        .cover_tile
                        .get_image(cover_type, tile.cover_graphic_index as usize)
                        .clone();
                    parent.spawn_bundle(SpriteBundle {
                        transform: cover_tranform,
                        texture: cover_texture,
                        ..Default::default()
                    });
                } else {
                    parent.spawn_bundle(SpriteBundle {
                        transform: cover_tranform,
                        ..Default::default()
                    });
                }
            })
            .insert(component);
    }
}

pub fn copy_from_tile_to_entity_component(component: &mut TileComponent, tile_component: &mut TileCoverComponent, tile: &Tile) {
    component.ground_type = tile.ground_type.clone();
    component.index = tile.index;
    component.cover_graphic_index = tile.cover_graphic_index;
    component.movement_ratio = tile.movement_ratio;
    component.position = tile.position.clone();
    component.graphic_position = tile.graphic_position.clone();
    component.permissions = tile.permissions.to_vec();
    component.thing_type = tile.thing_type.clone();
    component.stuff_type = tile.stuff_type.to_vec();
    component.alive_charactor_type = tile.alive_charactor_type.clone();
    component.dead_charactor_type = tile.dead_charactor_type.to_vec();
    component.effect_type = tile.effect_type.clone();

    tile_component.cover_type = tile.cover_type.clone();
}
