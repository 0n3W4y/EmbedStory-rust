use bevy::prelude::*;

use super::tile::CoverType;
use crate::components::tile_component::TileComponent;


pub fn change_cover_type_handler(
    mut commands: Commands,
    mut tile_query: Query<(Entity, &TileComponent, &mut Children),  With<TileComponent>>,
){
    for(entity, component, children) in tile_query.iter_mut(){
        let texture = match component.cover_type{
            CoverType::None => {
                let new_texture:Handle<Image> = Default::default();
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

/*
pub fn change_cover_type_handler(
    mut commands: Commands,
    mut tile_query: Query<(&TileComponent, &mut Children), (Changed<TileComponent>, With<TileComponent>)>,
    mut texture_query: Query<(&mut Handle<Image>, With<TileComponent>)>
){
    for(component, children) in tile_query.iter(){
        let new_texture:Handle<Image> = Default::default();
            
        if let Ok(mut texture) = texture_query.get_mut(*children.first().unwrap()) {
            texture = &mut new_texture;
        }
    }
}
*/