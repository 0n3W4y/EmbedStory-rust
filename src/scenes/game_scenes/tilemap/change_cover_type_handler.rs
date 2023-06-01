use bevy::prelude::*;

use crate::{components::tile_component::TileCoverComponent, materials::material_manager::MaterialManager};


pub fn change_cover_type_handler(
    mut tile_query: Query<(&TileCoverComponent, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>,), (Changed<TileCoverComponent>, With<TileCoverComponent>)>,
    material_manager: Res<MaterialManager>,
    
){
    for(component, mut sprite, mut texture_atlas_handle) in tile_query.iter_mut(){
        let new_texture: Handle<TextureAtlas> = material_manager.game_scene.tile.get_cover_atlas(&component.cover_type);
        *texture_atlas_handle = new_texture;
        sprite.index = component.cover_graphic_index as usize;
    }
}

