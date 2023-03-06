use bevy::prelude::*;

use crate::{components::thing_component::ThingComponent, resources::scene_manager::SceneManager};

use super::Thing;
use super::cleanup;

pub fn despawn(
    mut commands: Commands,
    mut things_query: Query<(Entity, &ThingComponent), With<ThingComponent>>,
    thing: &mut Thing
){
    let thing_id = thing.id;
    for (entity, component) in things_query.iter_mut(){
        if component.id == thing_id {
            cleanup::copy_from_entity_component_to_thing(thing, component);
            commands.entity(entity).despawn_recursive();
        }
    }
}