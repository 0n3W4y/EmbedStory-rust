use bevy::prelude::*;

use crate::components::thing_component::ThingComponent;

pub fn cleanup(
    mut commands: Commands,
    mut things_query: Query<Entity, With<ThingComponent>>,
){
    for entity in things_query.iter_mut(){
        commands.entity(entity).despawn_recursive();
    }
}