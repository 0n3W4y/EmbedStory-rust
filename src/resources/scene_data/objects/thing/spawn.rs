use bevy::prelude::*;

use crate::{materials::material_manager::MaterialManager, resources::scene_manager::SceneManager};
use crate::components::thing_component::ThingComponent;

use super::{draw::Z_POSITION, Thing};
use super::draw;

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

    let new_z_position = Z_POSITION + ((total_tiles as f32 - thing.tile_index as f32) / total_tiles as f32);
    let texture = material_manager
                .game_scene
                .things
                .get_image(thing_type, index as usize);
    let transform = Transform::from_xyz(x, y, new_z_position); // third layer;
    let mut component: ThingComponent = Default::default();
    draw::copy_from_thing_to_entity_component(&mut component, thing);

    commands
    .spawn_bundle(SpriteBundle {
        transform,
        texture,
        ..Default::default()
    })
    .insert(component);
}