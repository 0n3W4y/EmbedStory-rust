use bevy::prelude::*;

use crate::resources::scene_manager::SceneManager;
use crate::materials::material_manager::MaterialManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::scenes::game_scenes::tilemap::tile::CoverType;
use crate::components::tile_component::TileComponent;

pub const Z_POSTION_FOR_GROUND: f32 = 0.0;
pub const Z_POSITION_FOR_COVER: f32 = 0.1;

pub fn draw( 
    mut commands: Commands, 
    scene_manager: Res<SceneManager>,
    material_manager: Res<MaterialManager>
){
    let current_scene: &GameScene = scene_manager.get_current_game_scene();
    let tile_storage = current_scene.tilemap.get_tilemap_tile_storage();
    for tile in tile_storage.iter() {
        let x = tile.graphic_position.x;
        let y = tile.graphic_position.y;
        let ground_type = &tile.ground_type;        
        let ground_transform = Transform::from_xyz(x as f32, y as f32, Z_POSTION_FOR_GROUND);
        let ground_texture: Handle<Image> = material_manager.game_scene.ground_tile.get_image(ground_type).clone();
        

        commands.spawn_bundle(SpriteBundle{
            transform: ground_transform,
            texture: ground_texture,
            ..Default::default()
        })
        .with_children(|parent|{
            let cover_type = &tile.cover_type;
            let cover_tranform: Transform = Transform::from_xyz(0.0, 0.0, Z_POSITION_FOR_COVER);
            if *cover_type != CoverType::None{
                let cover_texture: Handle<Image> = material_manager.game_scene.cover_tile.get_image(cover_type, tile.cover_graphic_index as usize).clone();
                parent.spawn_bundle(SpriteBundle{
                    transform: cover_tranform,
                    texture: cover_texture,
                    ..Default::default()
                });
            } else {
                parent.spawn_bundle(SpriteBundle{
                    transform: cover_tranform,
                    ..Default::default()
                });
            }    
        })
        .insert(TileComponent{
            ground_type: tile.ground_type.clone(),
            cover_type: tile.cover_type.clone(),
            index: tile.index,
            cover_graphic_index: tile.cover_graphic_index,
            movement_ratio: tile.movement_ratio,

            position: tile.position.clone(),
            graphic_position: tile.graphic_position.clone(),

            permissions: tile.permissions.to_vec(),

            thing_type: tile.thing_type.clone(), // ( thing type, id of thing);
            stuff_type:  tile.stuff_type.clone(),
            character_type:  tile.character_type.clone(),
            effect_type: tile.effect_type.clone(),
        });
    };
}
