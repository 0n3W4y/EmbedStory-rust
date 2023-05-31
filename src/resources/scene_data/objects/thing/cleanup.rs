use bevy::prelude::*;

use crate::{components::thing_component::ThingComponent, resources::scene_manager::SceneManager};
use super::Thing;

pub fn cleanup(
    mut commands: Commands,
    mut things_query: Query<(Entity, &ThingComponent), With<ThingComponent>>,
    mut scene_manager: ResMut<SceneManager>,
){
    let scene = scene_manager.get_current_game_scene_mut();
    for (entity, component) in things_query.iter_mut(){
        let thing_id: usize = component.id;
        let thing = match scene.get_thing_by_id_mut(thing_id){
            Option::Some(v) => v,
            Option::None => {
                println!(
                    "thing.cleanup. Can not find thing: {:?}, with id:{:?}. So i created default() and store into things storage on scene",
                    component.thing_type,
                    component.id
                );
                let new_thing = Thing{id: component.id, ..Default::default()};
                scene.things.push(new_thing);
                let index = scene.things.len();
                &mut scene.things[index -1]
            }, 
        };
        copy_from_entity_component_to_thing(thing, component);    
        commands.entity(entity).despawn_recursive();
    }
}

pub fn copy_from_entity_component_to_thing( thing: &mut Thing, component: &ThingComponent){
        thing.thing_type = component.thing_type.clone();
        thing.id = component.id;
        thing.tile_index = component.tile_index;
        thing.position = component.position.clone();
        thing.graphic_index = component.graphic_index;
        thing.permissions = component.permissions.to_vec();
        thing.resists = component.resists.clone();
        thing.resists_cache = component.resists_cache.clone();
        thing.body_structure = component.body_structure.clone();
        thing.current_health_points = component.current_health_points;
        thing.total_health_points = component.total_health_points;
}