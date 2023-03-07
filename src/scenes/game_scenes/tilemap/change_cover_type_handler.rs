use bevy::prelude::*;

use super::tile::CoverType;
use crate::{components::tile_component::{TileComponent, tile_cover_component::TileCoverComponent}, materials::material_manager::MaterialManager};


pub fn change_cover_type_handler(
    mut tile_query: Query<(&TileComponent, &TileCoverComponent, &mut Children), (Changed<TileCoverComponent>, With<TileComponent>)>,
    mut texture_query: Query<(&mut Handle<Image>, With<TileComponent>)>,
    material_manager: Res<MaterialManager>,
    
){
    for(component, cover_component, children) in tile_query.iter_mut(){
        let new_texture: Handle<Image> = match cover_component.cover_type{
            CoverType::None => Default::default(),            
            _ => material_manager.game_scene.cover_tile.get_image(&cover_component.cover_type, component.cover_graphic_index as usize).clone(),  
        };
        if let Ok((mut texture, _)) = texture_query.get_mut(*children.first().unwrap()) {
            *texture = new_texture;
        }
    }
}

