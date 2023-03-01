use bevy::prelude::*;

use crate::components::thing_component::ThingComponent;

use super::Thing;

pub fn despawn(
    mut commands: Commands,
    mut things_query: Query<(Entity, &ThingComponent), With<ThingComponent>>,
    thing: &Thing
){
    let thing_id = thing.id;
    for (entity, component) in things_query.iter_mut(){
        if component.id == thing_id {
            commands.entity(entity).despawn_recursive();
        }
    }
}