use bevy::prelude::*;

use crate::{components::tile_component::TileComponent, materials::material_manager::MaterialManager};


pub fn change_cover_type_handler(
    mut tile_query: Query<(&TileComponent, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>,), (Changed<TileComponent>, With<TileComponent>)>,
    material_manager: Res<MaterialManager>,
    
){
    for(component, mut sprite, mut texture_atlas_handle) in tile_query.iter_mut(){
        let new_texture: Handle<TextureAtlas> = material_manager.game_scene.tile.get_cover_atlas(&component.cover_type);
        *texture_atlas_handle = new_texture;
        sprite.index = component.cover_graphic_index as usize;
    }
}

