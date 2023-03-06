use bevy::prelude::*;

use crate::{materials::material_manager::MaterialManager, resources::scene_manager::SceneManager};
use crate::components::thing_component::ThingComponent;

use super::{draw::Z_POSITION, Thing};

pub fn spawn(
    mut commands: Commands,
    scene_manager: Res<SceneManager>,
    material_manager: Res<MaterialManager>,
    thing: &Thing
){
    let x: f32 = thing.graphic_position.x;
    let y: f32 = thing.graphic_position.y;
    let index = thing.graphic_index;
    let thing_type = &thing.thing_type;
    let total_tiles = scene_manager.get_current_game_scene().tilemap.get_total_tiles();

    let new_z_position = Z_POSITION + thing.tile_index as f32 / total_tiles as f32;
    let texture = material_manager
                .game_scene
                .things
                .get_image(thing_type, index as usize);
    let transform = Transform::from_xyz(x, y, new_z_position); // third layer;
    let mut component: ThingComponent = Default::default();
    copy_from_thing_to_entity_component(&mut component, thing);

    commands
    .spawn_bundle(SpriteBundle {
        transform,
        texture,
        ..Default::default()
    })
    .insert(component);
}

pub fn copy_from_thing_to_entity_component(component: &mut ThingComponent, thing: &Thing){
    component.thing_type = thing.thing_type.clone();
    component.id = thing.id; 
    component.tile_index = thing.tile_index;
    component.position = thing.position.clone();
    component.graphic_position = thing.graphic_position.clone();
    component.graphic_index = thing.graphic_index;
    component.permissions = thing.permissions.to_vec();
    component.resists = thing.resists.to_vec();
    component.resists_cache = thing.resists_cache.to_vec();
    component.body_structure = thing.body_structure.to_vec();
    component.current_health_points = thing.current_health_points;
    component.total_health_points = thing.total_health_points;
}