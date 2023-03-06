use bevy::prelude::*;

use super::tile::CoverType;
use crate::{components::tile_component::TileComponent, materials::material_manager::MaterialManager};

/*
pub fn change_cover_type_handler(
    mut commands: Commands,
    mut tile_query: Query<(Entity, &TileComponent, &mut Children),  (Changed<tileComponent>, With<TileComponent>)>,
    material_manager: Res<MaterialManager>,
){
    for(entity, component, children) in tile_query.iter_mut(){
        let texture = match component.cover_type{
            CoverType::None => {
                let new_texture:Handle<Image> = material_manager.game_scene.cover_tile.get_image(&component.cover_type, 0);
                new_texture
            },
            _ => {
                let new_texture:Handle<Image> = Default::default();
                new_texture
            },
            /*
            CoverType::Water => {},
            CoverType::Flowers => {},
            CoverType::Grass => {},
            CoverType::Ice => {},
            CoverType::RockyRoad => {},
            CoverType::Sand => {},
            CoverType::Shallow => {},
            CoverType::Snow => {},
            CoverType::WoodenFloor => {},
            */
        };
        commands.entity(*children.first().unwrap()).despawn_recursive();

        let child = commands.spawn_bundle(SpriteBundle{
            texture,
            ..Default::default()
        }).id();
        commands.entity(entity).push_children(&[child]);
    }
}
*/

pub fn change_cover_type_handler(
    tile_query: Query<(&TileComponent, &mut Children), (Changed<TileComponent>, With<TileComponent>)>,
    mut texture_query: Query<(&mut Handle<Image>, With<TileComponent>)>,
    material_manager: Res<MaterialManager>,
){
    for(component, children) in tile_query.iter(){
        //let new_texture:Handle<Image> = material_manager.game_scene.cover_tile.get_image(&component.cover_type, 0).clone();
        let new_texture:Handle<Image> = Default::default();
        if let Ok((mut texture, _)) = texture_query.get_mut(*children.first().unwrap()) {
            *texture = new_texture;
        }

    }
}
