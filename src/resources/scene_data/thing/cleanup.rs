use bevy::prelude::*;

use crate::{components::{thing_component::{ThingComponent, ThingPermissionsComponent}, IdentificationComponent, PositionComponent, StatsComponent}, resources::scene_manager::SceneManager};

use super::Thing;

pub fn cleanup(
    mut commands: Commands,
    mut things_query: Query<(Entity, &IdentificationComponent, &PositionComponent, &StatsComponent, &ThingPermissionsComponent, &ThingComponent),  With<ThingComponent>>,
    mut scene_manager: ResMut<SceneManager>,
){
    let scene = scene_manager.get_current_game_scene_mut();
    scene.things.clear_all();
    for (
        entity, 
        identification, 
        position, 
        stats, 
        permissions, 
        thing_component
    ) in things_query.iter_mut(){
        let mut new_thing: Thing = Default::default();
        copy_from_components_to_thing(
            identification, 
            thing_component, 
            position, 
            stats, 
            permissions, 
            &mut new_thing
        );
        scene.things.store(new_thing);
        commands.entity(entity).despawn_recursive();
    }
}


pub fn copy_from_components_to_thing(
    identification_component: &IdentificationComponent,
    thing_component: &ThingComponent, 
    position_component: &PositionComponent, 
    attributes_component: &StatsComponent, 
    permissions_component: &ThingPermissionsComponent,
    thing: &mut Thing,
) {
    thing.thing_type = thing_component.thing_type.clone();
    thing.id = identification_component.id;
    thing.graphic_index = thing_component.graphic_index;
    thing.thing_defense_type = thing_component.thing_defense_type.clone();

    thing.position.x = position_component.position.x;
    thing.position.y = position_component.position.y;
    thing.permissions = permissions_component.permissions.to_vec();

    thing.attributes = attributes_component.attributes.clone();
    thing.attributes = attributes_component.attributes_cache.clone();

}

