use bevy::prelude::*;

use crate::components::charactor_component::CharactorComponent;


pub fn killed_charactor_handler(
    mut charactor_query: Query<(
        &mut CharactorComponent, 
        &mut TextureAtlasSprite,
    )>,
){
    //TODO: change sprite to dead, run timer to despawn creature;
    for (mut component, mut sprite) in charactor_query.iter_mut(){

    }
}