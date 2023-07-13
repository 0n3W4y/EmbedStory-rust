use bevy::prelude::*;

use crate::{components::thing_component::ThingComponent, resources::scene_manager::SceneManager};

pub fn cleanup(
    mut commands: Commands,
    mut things_query: Query<Entity, With<ThingComponent>>,
    mut scene_manager: ResMut<SceneManager>,
){
    let scene = scene_manager.get_current_game_scene_mut();
    for entity in things_query.iter_mut(){ 
        commands.entity(entity).despawn_recursive();
    }
}

